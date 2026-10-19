//! Recovered Clash Royale 160402020 libg loader: Speck64/128-CBC.
//!
//! This independent decoder preserves ELF layout and original relocation slots.
//! Evidence, source identity and instruction-translation differential validation
//! are recorded in docs/libg-cipher.md. It does not run game startup.
use std::{env, fs, io, path::Path, process::Command, time::Instant};
const GAME_SHA: &str = "347afc57c8d168bb67d0defa488a89f76297025cac1e8544b5182bb7e70ff433";
const BINDING_SHA: &str = "29f73c4325b635d46233ca040c0048cad83570b7cad0fb456099785140f1bb83";
fn fail(s: impl Into<String>) -> io::Error { io::Error::new(io::ErrorKind::InvalidData,s.into()) }
fn sha(path: &Path) -> io::Result<String> {
    let o=Command::new("sha256sum").arg(path).output()?;
    if !o.status.success(){return Err(fail("sha256sum failed"));}
    Ok(String::from_utf8_lossy(&o.stdout).split_whitespace().next().unwrap_or("").into())
}
fn u64le(b:&[u8],o:usize)->io::Result<u64>{Ok(u64::from_le_bytes(b.get(o..o+8).ok_or_else(||fail("truncated u64"))?.try_into().unwrap()))}
fn u32le(b:&[u8],o:usize)->io::Result<u32>{Ok(u32::from_le_bytes(b.get(o..o+4).ok_or_else(||fail("truncated u32"))?.try_into().unwrap()))}
fn take64(b:&[u8],p:&mut usize)->io::Result<u64>{let n=u64le(b,*p)?;*p+=8;Ok(n)}
fn speck_decrypt(block:[u8;8],keys:&[u32;27])->[u8;8]{
    // Original instruction order: words are stored y then x, little endian.
    let mut y=u32::from_le_bytes(block[..4].try_into().unwrap());
    let mut x=u32::from_le_bytes(block[4..].try_into().unwrap());
    for &key in keys.iter().rev(){y=(y^x).rotate_right(3);x=(x^key).wrapping_sub(y).rotate_left(8);}
    let mut out=[0;8];out[..4].copy_from_slice(&y.to_le_bytes());out[4..].copy_from_slice(&x.to_le_bytes());out
}
fn run()->io::Result<()>{
    let a:Vec<_>=env::args_os().collect();if a.len()!=4{return Err(fail("Usage: libg_cipher_decode ORIGINAL_LIBG DECODED_BINDING NEW_OUTPUT_ELF"));}
    let src=Path::new(&a[1]);let binding_path=Path::new(&a[2]);let dst=Path::new(&a[3]);
    if dst.exists(){return Err(fail("Output already exists; choose a new path"));}
    if sha(src)?!=GAME_SHA{return Err(fail("Original libg SHA256 mismatch"));}
    if sha(binding_path)?!=BINDING_SHA{return Err(fail("Decoded binding SHA256 mismatch"));}
    let mut game=fs::read(src)?;let binding=fs::read(binding_path)?;
    if game.get(..6)!=Some(&b"\x7fELF\x02\x01"[..]){return Err(fail("Expected little-endian ELF64"));}
    let phoff=u64le(&game,32)? as usize;let phsize=u16::from_le_bytes(game[54..56].try_into().unwrap()) as usize;let phnum=u16::from_le_bytes(game[56..58].try_into().unwrap()) as usize;
    let mut loads=Vec::new();for i in 0..phnum{let p=phoff+i*phsize;if u32le(&game,p)?==1{loads.push((u64le(&game,p+16)?,u64le(&game,p+8)?,u64le(&game,p+32)?));}}
    let start=Instant::now();let mut pos=0;let segment_count=take64(&binding,&mut pos)?;
    if segment_count!=2{return Err(fail("Unexpected protected segment count"));}
    let mut records=Vec::new();let mut total=0u64;
    for segment in 0..segment_count{
        let segva=take64(&binding,&mut pos)?;let segsize=take64(&binding,&mut pos)?;let protection=take64(&binding,&mut pos)?;let contexts=take64(&binding,&mut pos)?;
        if contexts>32 || protection!=5{return Err(fail("Unexpected protected segment descriptor"));}
        for context in 0..contexts{
            let mut keys=[0u32;27];let key_offset=pos;for k in &mut keys{*k=u32le(&binding,pos)?;pos+=4;}
            let ranges=u32le(&binding,pos)?;pos+=4;let scratch=take64(&binding,&mut pos)?;
            if ranges>32 || scratch!=0{return Err(fail("Unexpected range descriptor"));}
            for range in 0..ranges{
                let va=take64(&binding,&mut pos)?;let n=take64(&binding,&mut pos)?;let iv=take64(&binding,&mut pos)?.to_le_bytes();
                let end=va.checked_add(n).ok_or_else(||fail("range overflow"))?;
                if va<segva || end>segva+segsize || n%8!=0{return Err(fail("Protected range bounds/alignment mismatch"));}
                let &(loadva,loadoff,_)=loads.iter().find(|&&(v,_,sz)|va>=v&&end<=v+sz).ok_or_else(||fail("Range is not inside one file-backed ELF load segment"))?;
                let fileoff=(loadoff+va-loadva) as usize;let region=game.get_mut(fileoff..fileoff+n as usize).ok_or_else(||fail("Range outside source ELF"))?;
                let mut previous=iv;let mut changed=0u64;
                for block in region.chunks_exact_mut(8){
                    let ciphertext:[u8;8]=block.try_into().unwrap();let plain=speck_decrypt(ciphertext,&keys);
                    for i in 0..8{let value=plain[i]^previous[i];changed+=u64::from(block[i]!=value);block[i]=value;}
                    previous=ciphertext;
                }
                total+=n;
                records.push(format!("{{\"segment\":{segment},\"context\":{context},\"range\":{range},\"key_binding_offset\":\"0x{key_offset:x}\",\"virtual_address\":\"0x{va:x}\",\"file_offset\":\"0x{fileoff:x}\",\"size\":{n},\"iv\":\"{:016x}\",\"changed_bytes\":{changed}}}",u64::from_be_bytes(iv)));
            }
        }
    }
    if pos!=binding.len(){return Err(fail("Unconsumed binding bytes"));}
    let mut output=fs::OpenOptions::new().write(true).create_new(true).open(dst)?;use std::io::Write;output.write_all(&game)?;drop(output);
    println!("{{\"source\":{:?},\"source_sha256\":\"{GAME_SHA}\",\"binding\":{:?},\"binding_sha256\":\"{BINDING_SHA}\",\"derived\":{:?},\"derived_sha256\":\"{}\",\"method\":\"Recovered Speck64/128-CBC, native-expanded roundkeys, original ELF layout and relocation slots retained\",\"processed_bytes\":{total},\"seconds\":{},\"ranges\":[{}],\"limitations\":[\"This decodes protected file ranges; no Android startup, battle behavior or Rust game engine equivalence is established.\"]}}",src.to_string_lossy(),binding_path.to_string_lossy(),dst.to_string_lossy(),sha(dst)?,start.elapsed().as_secs_f64(),records.join(","));
    Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1);}}
