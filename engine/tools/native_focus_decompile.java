// Focused, bounded recovery. No global autoanalysis is required or invoked.
// @category ClashResearch
import ghidra.app.script.GhidraScript;
import ghidra.app.cmd.disassemble.DisassembleCommand;
import ghidra.app.decompiler.*;
import ghidra.framework.Application;
import ghidra.program.model.address.*;
import ghidra.program.model.listing.*;
import ghidra.program.model.symbol.SourceType;
import com.google.gson.GsonBuilder;
import java.nio.file.*;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.util.*;

public class native_focus_decompile extends GhidraScript {
    private static String hash(byte[] data) throws Exception {
        byte[] digest = MessageDigest.getInstance("SHA-256").digest(data);
        StringBuilder out = new StringBuilder();
        for (byte b : digest) out.append(String.format("%02x", b & 255));
        return out.toString();
    }
    public void run() throws Exception {
        String[] args = getScriptArgs();
        if (args.length != 6) throw new IllegalArgumentException("targets.tsv output_dir original_elf script_path all_fdes.tsv import_thunks.tsv");
        Path targets = Paths.get(args[0]), output = Paths.get(args[1]);
        Files.createDirectories(output);
        Map<String,Object> manifest = new LinkedHashMap<>();
        manifest.put("schema", "clash-focused-ghidra-v1");
        manifest.put("source_elf", args[2]);
        manifest.put("source_sha256", hash(Files.readAllBytes(Paths.get(args[2]))));
        manifest.put("script_sha256", hash(Files.readAllBytes(Paths.get(args[3]))));
        manifest.put("target_table_sha256", hash(Files.readAllBytes(targets)));
        manifest.put("original_fde_table_sha256", hash(Files.readAllBytes(Paths.get(args[4]))));
        manifest.put("import_thunks_table_sha256", hash(Files.readAllBytes(Paths.get(args[5]))));
        manifest.put("ghidra_version", Application.getApplicationVersion());
        manifest.put("language", currentProgram.getLanguageID().toString());
        manifest.put("compiler_spec", currentProgram.getCompilerSpec().getCompilerSpecID().toString());
        manifest.put("image_base", currentProgram.getImageBase().toString());
        manifest.put("analysis_mode", "ELF import with -noanalysis; restricted flow disassembly inside original FDE ranges; exact FDE function bodies");
        manifest.put("limits", Arrays.asList("Pseudocode is an inferred representation, not original C++ source.", "Unwind ranges establish bounds, not semantic names, types, ownership, or complete behavior.", "No interprocedural autoanalysis, runtime execution, or inferred type propagation was performed.", "Imported ELF relocations may affect memory bytes; both ELF file and imported-memory function byte hashes are recorded.", "Calls outside selected FDE ranges remain unanalysed and may have incomplete prototypes.", "An explicit jump outside a selected FDE is declared a separate function only when its destination is another exact original FDE start. An unconditional such jump gets an analysis-only CALL_RETURN flow override to stop decompiler traversal. The original instructions and bytes are retained in the assembly and file hashes."));
        Map<Long,Long> fdeBounds = new HashMap<>();
        for (String line : Files.readAllLines(Paths.get(args[4]), StandardCharsets.UTF_8)) {
            if (line.startsWith("start") || line.trim().isEmpty()) continue;
            String[] cols = line.split("\t");
            fdeBounds.put(Long.decode(cols[0]), Long.decode(cols[1]));
        }
        List<Map<String,Object>> records = new ArrayList<>();
        manifest.put("functions", records);
        List<String> noReturnImports = new ArrayList<>();
        Set<String> knownNoReturn = new HashSet<>(Arrays.asList("_ZSt9terminatev", "abort", "exit", "_exit", "__stack_chk_fail", "__cxa_throw", "__cxa_rethrow", "_ZNSt6__ndk120__throw_system_errorEiPKc", "__assert2", "__assert_fail"));
        for (String line : Files.readAllLines(Paths.get(args[5]), StandardCharsets.UTF_8)) {
            if (line.startsWith("rva") || line.trim().isEmpty()) continue;
            String[] cols = line.split("\t");
            if (!knownNoReturn.contains(cols[1])) continue;
            Address start = currentProgram.getImageBase().add(Long.decode(cols[0]));
            Function function = currentProgram.getFunctionManager().getFunctionAt(start);
            if (function == null) function = currentProgram.getFunctionManager().createFunction("Import_"+cols[1], start, new AddressSet(start,start.add(15)), SourceType.USER_DEFINED);
            function.setNoReturn(true);
            noReturnImports.add(cols[0]+" "+cols[1]);
        }
        manifest.put("known_import_noreturn_declarations", noReturnImports);
        DecompInterface decompiler = new DecompInterface();
        decompiler.toggleCCode(true);
        decompiler.toggleSyntaxTree(true);
        decompiler.setSimplificationStyle("decompile");
        if (!decompiler.openProgram(currentProgram)) throw new RuntimeException(decompiler.getLastMessage());
        for (String line : Files.readAllLines(targets, StandardCharsets.UTF_8)) {
            if (line.startsWith("start") || line.trim().isEmpty()) continue;
            String[] cols = line.split("\t");
            long rva = Long.decode(cols[0]), endRva = Long.decode(cols[1]);
            Address start = currentProgram.getImageBase().add(rva);
            Address end = currentProgram.getImageBase().add(endRva - 1);
            AddressSet body = new AddressSet(start, end);
            Map<String,Object> record = new LinkedHashMap<>(); records.add(record);
            record.put("rva", String.format("0x%x", rva));
            record.put("end_rva_exclusive", String.format("0x%x", endRva));
            record.put("address", start.toString());
            record.put("fde_record_rva", cols[3]); record.put("cie_record_rva", cols[4]);
            record.put("size", endRva-rva);
            byte[] bytes = new byte[(int)(endRva-rva)];
            currentProgram.getMemory().getBytes(start, bytes);
            record.put("imported_memory_bytes_sha256", hash(bytes));
            String stem = String.format("rva_%x", rva);
            println("FOCUS " + stem + " bytes=" + bytes.length);
            try {
                // Restrict traversal to the original unwind range. Never recursively
                // disassemble a callee or create an unconstrained function body.
                DisassembleCommand command = new DisassembleCommand(start, body, true);
                record.put("disassembly_ok", command.applyTo(currentProgram, monitor));
                record.put("disassembly_status", command.getStatusMsg());
                Function existing = currentProgram.getFunctionManager().getFunctionAt(start);
                if (existing != null) currentProgram.getFunctionManager().removeFunction(start);
                Function function = currentProgram.getFunctionManager().createFunction("Recovered_" + stem, start, body, SourceType.USER_DEFINED);
                StringBuilder assembly = new StringBuilder();
                int instructionCount=0, instructionBytes=0;
                InstructionIterator it = currentProgram.getListing().getInstructions(body, true);
                while (it.hasNext()) {
                    Instruction ins = it.next(); instructionCount++; instructionBytes += ins.getLength();
                    assembly.append(ins.getAddress()).append("  ").append(ins.toString()).append('\n');
                }
                List<String> boundaryDeclarations = new ArrayList<>();
                List<String> boundaryOverrides = new ArrayList<>();
                it = currentProgram.getListing().getInstructions(body, true);
                while (it.hasNext()) {
                    Instruction ins = it.next();
                    if (!ins.getFlowType().isJump()) continue;
                    for (Address target : ins.getFlows()) {
                        if (body.contains(target)) continue;
                        long targetRva = target.subtract(currentProgram.getImageBase());
                        Long targetEnd = fdeBounds.get(targetRva);
                        if (targetEnd != null && currentProgram.getFunctionManager().getFunctionAt(target) == null) {
                            AddressSet externalBody = new AddressSet(target, currentProgram.getImageBase().add(targetEnd-1));
                            currentProgram.getFunctionManager().createFunction("Boundary_rva_"+Long.toHexString(targetRva), target, externalBody, SourceType.USER_DEFINED);
                            boundaryDeclarations.add(String.format("0x%x", targetRva));
                        }
                        if (targetEnd != null && !ins.getFlowType().isConditional()) {
                            ins.setFlowOverride(FlowOverride.CALL_RETURN);
                            boundaryOverrides.add(ins.getAddress().toString()+" -> "+target.toString());
                        }
                    }
                }
                record.put("outgoing_jump_fde_declarations", boundaryDeclarations);
                record.put("analysis_only_call_return_overrides", boundaryOverrides);
                Files.writeString(output.resolve(stem+".asm"), assembly, StandardCharsets.UTF_8);
                record.put("instruction_count", instructionCount); record.put("instruction_bytes", instructionBytes);
                DecompileResults result = decompiler.decompileFunction(function, 60, monitor);
                record.put("decompile_completed", result.decompileCompleted());
                record.put("decompile_error", result.getErrorMessage());
                if (result.getDecompiledFunction() != null) {
                    String code = result.getDecompiledFunction().getC();
                    Files.writeString(output.resolve(stem+".c"), code, StandardCharsets.UTF_8);
                    record.put("pseudocode_file", stem+".c");
                    record.put("pseudocode_sha256", hash(code.getBytes(StandardCharsets.UTF_8)));
                }
            } catch (Exception error) { record.put("error", error.toString()); println("FOCUS ERROR " + error); }
            Files.writeString(output.resolve("manifest.json"), new GsonBuilder().setPrettyPrinting().create().toJson(manifest), StandardCharsets.UTF_8);
            decompiler.flushCache();
        }
        decompiler.dispose();
    }
}
