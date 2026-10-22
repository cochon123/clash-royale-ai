//! Observed operations in the native LogicBattleDeck function-pointer table.
//!
//! Array names deliberately identify original object offsets. Whether the two
//! arrays mean hand, queue or another deck representation is still unresolved.
//! The u64 entries preserve native pointer identity; pointees are not accessed.

pub const SWAP_ARRAY_20_RVA: u64 = 0x00fe_c180;
pub const FIND_ARRAY_40_RVA: u64 = 0x00fe_c49c;
pub const ALL_NONZERO_ARRAY_40_RVA: u64 = 0x00fe_c320;
pub const ANY_NONZERO_ARRAY_40_RVA: u64 = 0x00fe_c370;
pub const LOAD_ARRAY_40_RVA: u64 = 0x00fe_c3c0;
pub const LOAD_FIELD_08_RVA: u64 = 0x00fe_da80;
pub const LOAD_FIELD_2C_RVA: u64 = 0x00fe_da88;
pub const LOAD_FIELD_4C_RVA: u64 = 0x00fe_da90;
pub const DECRYPTED_SOURCE_SHA256: &str =
    "79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexOutsideArray {
    pub index: i32,
    pub length: usize,
}

/// Port of RVA 0xfec180: exchange two 8-byte identities in the array at +0x20.
///
/// The native method sign-extends both i32 indices and has no bounds checks.
/// This safe boundary accepts its valid allocated-index domain; invalid indices
/// return an error without changing either entry.
#[inline]
pub fn swap_array_20(
    entries: &mut [u64],
    first: i32,
    second: i32,
) -> Result<(), IndexOutsideArray> {
    let checked = |index: i32| {
        usize::try_from(index)
            .ok()
            .filter(|&value| value < entries.len())
            .ok_or(IndexOutsideArray {
                index,
                length: entries.len(),
            })
    };
    let first = checked(first)?;
    let second = checked(second)?;
    entries.swap(first, second);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountOutsideArray {
    pub count: i32,
    pub available: usize,
}

/// Port of RVA 0xfec49c: return the first matching identity, or signed i32 -1.
///
/// The original uses the signed count at object+0x4c. Counts <= 0 return -1
/// without reading array memory. Duplicate and zero identities participate in
/// ordinary pointer equality. A positive count beyond supplied memory is
/// rejected at the safe boundary.
#[inline]
pub fn find_array_40(entries: &[u64], count: i32, needle: u64) -> Result<i32, CountOutsideArray> {
    if count <= 0 {
        return Ok(-1);
    }
    let count_usize = count as usize;
    if count_usize > entries.len() {
        return Err(CountOutsideArray {
            count,
            available: entries.len(),
        });
    }
    Ok(entries[..count_usize]
        .iter()
        .position(|&entry| entry == needle)
        .map(|index| index as i32)
        .unwrap_or(-1))
}

/// Port of RVA 0xfec320. True when every counted identity is nonzero.
///
/// A signed count at object+0x4c that is less than 1 returns true without
/// reading the array. The original stops at the first zero. A positive count
/// beyond the supplied slice is rejected.
#[inline]
pub fn all_nonzero_array_40(entries: &[u64], count: i32) -> Result<bool, CountOutsideArray> {
    counted(entries, count).map(|window| window.iter().all(|entry| *entry != 0))
}

/// Port of RVA 0xfec370. True when any counted identity is nonzero.
///
/// A signed count less than 1 returns false without reading the array.
#[inline]
pub fn any_nonzero_array_40(entries: &[u64], count: i32) -> Result<bool, CountOutsideArray> {
    counted(entries, count).map(|window| window.iter().any(|entry| *entry != 0))
}

fn counted(entries: &[u64], count: i32) -> Result<&[u64], CountOutsideArray> {
    if count <= 0 {
        return Ok(&[]);
    }
    let count_usize = count as usize;
    if count_usize > entries.len() {
        return Err(CountOutsideArray {
            count,
            available: entries.len(),
        });
    }
    Ok(&entries[..count_usize])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadArray40Error {
    IndexOutsideCount { index: i32, count: i32 },
    IndexOutsideStorage { index: i32, available: usize },
}

/// Port of the valid path at RVA 0xfec3c0.
///
/// The original accepts `0 <= index < signed count`, then loads that identity.
/// An invalid index calls an assertion helper and still performs the load.
/// This boundary returns an error instead of reading outside the count or the
/// supplied slice.
#[inline]
pub fn load_array_40(entries: &[u64], count: i32, index: i32) -> Result<u64, LoadArray40Error> {
    if index < 0 || index >= count {
        return Err(LoadArray40Error::IndexOutsideCount { index, count });
    }
    let index_usize = index as usize;
    if index_usize >= entries.len() {
        return Err(LoadArray40Error::IndexOutsideStorage {
            index,
            available: entries.len(),
        });
    }
    Ok(entries[index_usize])
}

/// Port of RVA 0xfeda80, preserving all bits of the native pointer at +0x08.
#[inline]
pub fn load_field_08(object_prefix: &[u8; 0x50]) -> u64 {
    u64::from_le_bytes(object_prefix[0x08..0x10].try_into().unwrap())
}

/// Port of RVA 0xfeda88, preserving all bits of the native u32 field.
#[inline]
pub fn load_field_2c(object_prefix: &[u8; 0x50]) -> u32 {
    u32::from_le_bytes(object_prefix[0x2c..0x30].try_into().unwrap())
}

/// Port of RVA 0xfeda90. Search interprets this field's bits as a signed i32.
#[inline]
pub fn load_field_4c(object_prefix: &[u8; 0x50]) -> u32 {
    u32::from_le_bytes(object_prefix[0x4c..0x50].try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::{
        all_nonzero_array_40, any_nonzero_array_40, find_array_40, load_array_40, load_field_08,
        swap_array_20, LoadArray40Error,
    };

    #[test]
    fn predicates_match_the_signed_count_domain() {
        let entries = [1u64, 0, 4];
        assert_eq!(all_nonzero_array_40(&entries, 0), Ok(true));
        assert_eq!(all_nonzero_array_40(&entries, -2), Ok(true));
        assert_eq!(all_nonzero_array_40(&entries, 1), Ok(true));
        assert_eq!(all_nonzero_array_40(&entries, 2), Ok(false));
        assert_eq!(all_nonzero_array_40(&[7, 9], 2), Ok(true));
        assert_eq!(any_nonzero_array_40(&entries, 0), Ok(false));
        assert_eq!(any_nonzero_array_40(&entries, -1), Ok(false));
        assert_eq!(any_nonzero_array_40(&[0, 0, 5], 2), Ok(false));
        assert_eq!(any_nonzero_array_40(&[0, 0, 5], 3), Ok(true));
        assert!(all_nonzero_array_40(&entries, 4).is_err());
    }

    #[test]
    fn indexed_load_rejects_the_assertion_path() {
        let entries = [10u64, 20];
        assert_eq!(load_array_40(&entries, 2, 1), Ok(20));
        assert_eq!(
            load_array_40(&entries, 2, -1),
            Err(LoadArray40Error::IndexOutsideCount {
                index: -1,
                count: 2
            })
        );
        assert_eq!(
            load_array_40(&entries, 1, 1),
            Err(LoadArray40Error::IndexOutsideCount { index: 1, count: 1 })
        );
        assert_eq!(
            load_array_40(&entries, 3, 2),
            Err(LoadArray40Error::IndexOutsideStorage {
                index: 2,
                available: 2
            })
        );
    }

    #[test]
    fn field_and_search_edges_stay_in_the_verified_domain() {
        let mut prefix = [0u8; 0x50];
        prefix[0x08..0x10].copy_from_slice(&0x0102_0304_0506_0708u64.to_le_bytes());
        assert_eq!(load_field_08(&prefix), 0x0102_0304_0506_0708);
        assert_eq!(find_array_40(&[4, 4, 0], 0, 4), Ok(-1));
        assert_eq!(find_array_40(&[4, 4, 0], 3, 0), Ok(2));
        let mut entries = [1u64, 2, 3];
        swap_array_20(&mut entries, 0, 0).unwrap();
        assert_eq!(entries, [1, 2, 3]);
        assert!(swap_array_20(&mut entries, -1, 0).is_err());
    }
}
