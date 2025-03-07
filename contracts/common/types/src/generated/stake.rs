// Generated by Molecule 0.7.2

use super::basic::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct StakeInfo(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for StakeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for StakeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for StakeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "is_increase", self.is_increase())?;
        write!(f, ", {}: {}", "amount", self.amount())?;
        write!(
            f,
            ", {}: {}",
            "inauguration_epoch",
            self.inauguration_epoch()
        )?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for StakeInfo {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            49, 0, 0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        StakeInfo::new_unchecked(v.into())
    }
}
impl StakeInfo {
    pub const FIELD_COUNT: usize = 3;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn is_increase(&self) -> Byte {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Byte::new_unchecked(self.0.slice(start..end))
    }
    pub fn amount(&self) -> Byte16 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        Byte16::new_unchecked(self.0.slice(start..end))
    }
    pub fn inauguration_epoch(&self) -> Byte8 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[16..]) as usize;
            Byte8::new_unchecked(self.0.slice(start..end))
        } else {
            Byte8::new_unchecked(self.0.slice(start..))
        }
    }
    pub fn as_reader<'r>(&'r self) -> StakeInfoReader<'r> {
        StakeInfoReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for StakeInfo {
    type Builder = StakeInfoBuilder;
    const NAME: &'static str = "StakeInfo";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        StakeInfo(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        StakeInfoReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        StakeInfoReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .is_increase(self.is_increase())
            .amount(self.amount())
            .inauguration_epoch(self.inauguration_epoch())
    }
}
#[derive(Clone, Copy)]
pub struct StakeInfoReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for StakeInfoReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for StakeInfoReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for StakeInfoReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "is_increase", self.is_increase())?;
        write!(f, ", {}: {}", "amount", self.amount())?;
        write!(
            f,
            ", {}: {}",
            "inauguration_epoch",
            self.inauguration_epoch()
        )?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> StakeInfoReader<'r> {
    pub const FIELD_COUNT: usize = 3;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn is_increase(&self) -> ByteReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        ByteReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn amount(&self) -> Byte16Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        Byte16Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn inauguration_epoch(&self) -> Byte8Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[16..]) as usize;
            Byte8Reader::new_unchecked(&self.as_slice()[start..end])
        } else {
            Byte8Reader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for StakeInfoReader<'r> {
    type Entity = StakeInfo;
    const NAME: &'static str = "StakeInfoReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        StakeInfoReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len == molecule::NUMBER_SIZE && Self::FIELD_COUNT == 0 {
            return Ok(());
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE * 2, slice_len);
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % molecule::NUMBER_SIZE != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        if slice_len < offset_first {
            return ve!(Self, HeaderIsBroken, offset_first, slice_len);
        }
        let field_count = offset_first / molecule::NUMBER_SIZE - 1;
        if field_count < Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        } else if !compatible && field_count > Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        };
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..offset_first]
            .chunks_exact(molecule::NUMBER_SIZE)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        ByteReader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        Byte16Reader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        Byte8Reader::verify(&slice[offsets[2]..offsets[3]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct StakeInfoBuilder {
    pub(crate) is_increase: Byte,
    pub(crate) amount: Byte16,
    pub(crate) inauguration_epoch: Byte8,
}
impl StakeInfoBuilder {
    pub const FIELD_COUNT: usize = 3;
    pub fn is_increase(mut self, v: Byte) -> Self {
        self.is_increase = v;
        self
    }
    pub fn amount(mut self, v: Byte16) -> Self {
        self.amount = v;
        self
    }
    pub fn inauguration_epoch(mut self, v: Byte8) -> Self {
        self.inauguration_epoch = v;
        self
    }
}
impl molecule::prelude::Builder for StakeInfoBuilder {
    type Entity = StakeInfo;
    const NAME: &'static str = "StakeInfoBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.is_increase.as_slice().len()
            + self.amount.as_slice().len()
            + self.inauguration_epoch.as_slice().len()
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.is_increase.as_slice().len();
        offsets.push(total_size);
        total_size += self.amount.as_slice().len();
        offsets.push(total_size);
        total_size += self.inauguration_epoch.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.is_increase.as_slice())?;
        writer.write_all(self.amount.as_slice())?;
        writer.write_all(self.inauguration_epoch.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        StakeInfo::new_unchecked(inner.into())
    }
}
#[derive(Clone)]
pub struct StakeLockArgs(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for StakeLockArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for StakeLockArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for StakeLockArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "stake_smt_type_id", self.stake_smt_type_id())?;
        write!(f, ", {}: {}", "staker_identity", self.staker_identity())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for StakeLockArgs {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            64, 0, 0, 0, 12, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ];
        StakeLockArgs::new_unchecked(v.into())
    }
}
impl StakeLockArgs {
    pub const FIELD_COUNT: usize = 2;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn stake_smt_type_id(&self) -> Byte32 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Byte32::new_unchecked(self.0.slice(start..end))
    }
    pub fn staker_identity(&self) -> Identity {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[12..]) as usize;
            Identity::new_unchecked(self.0.slice(start..end))
        } else {
            Identity::new_unchecked(self.0.slice(start..))
        }
    }
    pub fn as_reader<'r>(&'r self) -> StakeLockArgsReader<'r> {
        StakeLockArgsReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for StakeLockArgs {
    type Builder = StakeLockArgsBuilder;
    const NAME: &'static str = "StakeLockArgs";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        StakeLockArgs(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        StakeLockArgsReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        StakeLockArgsReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .stake_smt_type_id(self.stake_smt_type_id())
            .staker_identity(self.staker_identity())
    }
}
#[derive(Clone, Copy)]
pub struct StakeLockArgsReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for StakeLockArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for StakeLockArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for StakeLockArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "stake_smt_type_id", self.stake_smt_type_id())?;
        write!(f, ", {}: {}", "staker_identity", self.staker_identity())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> StakeLockArgsReader<'r> {
    pub const FIELD_COUNT: usize = 2;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn stake_smt_type_id(&self) -> Byte32Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn staker_identity(&self) -> IdentityReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[12..]) as usize;
            IdentityReader::new_unchecked(&self.as_slice()[start..end])
        } else {
            IdentityReader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for StakeLockArgsReader<'r> {
    type Entity = StakeLockArgs;
    const NAME: &'static str = "StakeLockArgsReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        StakeLockArgsReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len == molecule::NUMBER_SIZE && Self::FIELD_COUNT == 0 {
            return Ok(());
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE * 2, slice_len);
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % molecule::NUMBER_SIZE != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        if slice_len < offset_first {
            return ve!(Self, HeaderIsBroken, offset_first, slice_len);
        }
        let field_count = offset_first / molecule::NUMBER_SIZE - 1;
        if field_count < Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        } else if !compatible && field_count > Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        };
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..offset_first]
            .chunks_exact(molecule::NUMBER_SIZE)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        Byte32Reader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        IdentityReader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct StakeLockArgsBuilder {
    pub(crate) stake_smt_type_id: Byte32,
    pub(crate) staker_identity: Identity,
}
impl StakeLockArgsBuilder {
    pub const FIELD_COUNT: usize = 2;
    pub fn stake_smt_type_id(mut self, v: Byte32) -> Self {
        self.stake_smt_type_id = v;
        self
    }
    pub fn staker_identity(mut self, v: Identity) -> Self {
        self.staker_identity = v;
        self
    }
}
impl molecule::prelude::Builder for StakeLockArgsBuilder {
    type Entity = StakeLockArgs;
    const NAME: &'static str = "StakeLockArgsBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.stake_smt_type_id.as_slice().len()
            + self.staker_identity.as_slice().len()
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.stake_smt_type_id.as_slice().len();
        offsets.push(total_size);
        total_size += self.staker_identity.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.stake_smt_type_id.as_slice())?;
        writer.write_all(self.staker_identity.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        StakeLockArgs::new_unchecked(inner.into())
    }
}
#[derive(Clone)]
pub struct StakeAtLockCellData(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for StakeAtLockCellData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for StakeAtLockCellData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for StakeAtLockCellData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "amount", self.amount())?;
        write!(f, ", {}: {}", "version", self.version())?;
        write!(f, ", {}: {}", "l1_address", self.l1_address())?;
        write!(f, ", {}: {}", "l2_address", self.l2_address())?;
        write!(f, ", {}: {}", "stake_info", self.stake_info())?;
        write!(
            f,
            ", {}: {}",
            "checkpoint_type_id",
            self.checkpoint_type_id()
        )?;
        write!(f, ", {}: {}", "xudt_type_id", self.xudt_type_id())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for StakeAtLockCellData {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            202, 0, 0, 0, 32, 0, 0, 0, 48, 0, 0, 0, 49, 0, 0, 0, 69, 0, 0, 0, 89, 0, 0, 0, 138, 0,
            0, 0, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];
        StakeAtLockCellData::new_unchecked(v.into())
    }
}
impl StakeAtLockCellData {
    pub const FIELD_COUNT: usize = 7;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn amount(&self) -> Byte16 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Byte16::new_unchecked(self.0.slice(start..end))
    }
    pub fn version(&self) -> Byte {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        Byte::new_unchecked(self.0.slice(start..end))
    }
    pub fn l1_address(&self) -> Identity {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        let end = molecule::unpack_number(&slice[16..]) as usize;
        Identity::new_unchecked(self.0.slice(start..end))
    }
    pub fn l2_address(&self) -> Identity {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[16..]) as usize;
        let end = molecule::unpack_number(&slice[20..]) as usize;
        Identity::new_unchecked(self.0.slice(start..end))
    }
    pub fn stake_info(&self) -> StakeInfo {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[20..]) as usize;
        let end = molecule::unpack_number(&slice[24..]) as usize;
        StakeInfo::new_unchecked(self.0.slice(start..end))
    }
    pub fn checkpoint_type_id(&self) -> Byte32 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[24..]) as usize;
        let end = molecule::unpack_number(&slice[28..]) as usize;
        Byte32::new_unchecked(self.0.slice(start..end))
    }
    pub fn xudt_type_id(&self) -> Byte32 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[28..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[32..]) as usize;
            Byte32::new_unchecked(self.0.slice(start..end))
        } else {
            Byte32::new_unchecked(self.0.slice(start..))
        }
    }
    pub fn as_reader<'r>(&'r self) -> StakeAtLockCellDataReader<'r> {
        StakeAtLockCellDataReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for StakeAtLockCellData {
    type Builder = StakeAtLockCellDataBuilder;
    const NAME: &'static str = "StakeAtLockCellData";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        StakeAtLockCellData(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        StakeAtLockCellDataReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        StakeAtLockCellDataReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .amount(self.amount())
            .version(self.version())
            .l1_address(self.l1_address())
            .l2_address(self.l2_address())
            .stake_info(self.stake_info())
            .checkpoint_type_id(self.checkpoint_type_id())
            .xudt_type_id(self.xudt_type_id())
    }
}
#[derive(Clone, Copy)]
pub struct StakeAtLockCellDataReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for StakeAtLockCellDataReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for StakeAtLockCellDataReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for StakeAtLockCellDataReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "amount", self.amount())?;
        write!(f, ", {}: {}", "version", self.version())?;
        write!(f, ", {}: {}", "l1_address", self.l1_address())?;
        write!(f, ", {}: {}", "l2_address", self.l2_address())?;
        write!(f, ", {}: {}", "stake_info", self.stake_info())?;
        write!(
            f,
            ", {}: {}",
            "checkpoint_type_id",
            self.checkpoint_type_id()
        )?;
        write!(f, ", {}: {}", "xudt_type_id", self.xudt_type_id())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> StakeAtLockCellDataReader<'r> {
    pub const FIELD_COUNT: usize = 7;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn amount(&self) -> Byte16Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Byte16Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn version(&self) -> ByteReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        ByteReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn l1_address(&self) -> IdentityReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        let end = molecule::unpack_number(&slice[16..]) as usize;
        IdentityReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn l2_address(&self) -> IdentityReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[16..]) as usize;
        let end = molecule::unpack_number(&slice[20..]) as usize;
        IdentityReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn stake_info(&self) -> StakeInfoReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[20..]) as usize;
        let end = molecule::unpack_number(&slice[24..]) as usize;
        StakeInfoReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn checkpoint_type_id(&self) -> Byte32Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[24..]) as usize;
        let end = molecule::unpack_number(&slice[28..]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn xudt_type_id(&self) -> Byte32Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[28..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[32..]) as usize;
            Byte32Reader::new_unchecked(&self.as_slice()[start..end])
        } else {
            Byte32Reader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for StakeAtLockCellDataReader<'r> {
    type Entity = StakeAtLockCellData;
    const NAME: &'static str = "StakeAtLockCellDataReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        StakeAtLockCellDataReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len == molecule::NUMBER_SIZE && Self::FIELD_COUNT == 0 {
            return Ok(());
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE * 2, slice_len);
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % molecule::NUMBER_SIZE != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        if slice_len < offset_first {
            return ve!(Self, HeaderIsBroken, offset_first, slice_len);
        }
        let field_count = offset_first / molecule::NUMBER_SIZE - 1;
        if field_count < Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        } else if !compatible && field_count > Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        };
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..offset_first]
            .chunks_exact(molecule::NUMBER_SIZE)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        Byte16Reader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        ByteReader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        IdentityReader::verify(&slice[offsets[2]..offsets[3]], compatible)?;
        IdentityReader::verify(&slice[offsets[3]..offsets[4]], compatible)?;
        StakeInfoReader::verify(&slice[offsets[4]..offsets[5]], compatible)?;
        Byte32Reader::verify(&slice[offsets[5]..offsets[6]], compatible)?;
        Byte32Reader::verify(&slice[offsets[6]..offsets[7]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct StakeAtLockCellDataBuilder {
    pub(crate) amount: Byte16,
    pub(crate) version: Byte,
    pub(crate) l1_address: Identity,
    pub(crate) l2_address: Identity,
    pub(crate) stake_info: StakeInfo,
    pub(crate) checkpoint_type_id: Byte32,
    pub(crate) xudt_type_id: Byte32,
}
impl StakeAtLockCellDataBuilder {
    pub const FIELD_COUNT: usize = 7;
    pub fn amount(mut self, v: Byte16) -> Self {
        self.amount = v;
        self
    }
    pub fn version(mut self, v: Byte) -> Self {
        self.version = v;
        self
    }
    pub fn l1_address(mut self, v: Identity) -> Self {
        self.l1_address = v;
        self
    }
    pub fn l2_address(mut self, v: Identity) -> Self {
        self.l2_address = v;
        self
    }
    pub fn stake_info(mut self, v: StakeInfo) -> Self {
        self.stake_info = v;
        self
    }
    pub fn checkpoint_type_id(mut self, v: Byte32) -> Self {
        self.checkpoint_type_id = v;
        self
    }
    pub fn xudt_type_id(mut self, v: Byte32) -> Self {
        self.xudt_type_id = v;
        self
    }
}
impl molecule::prelude::Builder for StakeAtLockCellDataBuilder {
    type Entity = StakeAtLockCellData;
    const NAME: &'static str = "StakeAtLockCellDataBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.amount.as_slice().len()
            + self.version.as_slice().len()
            + self.l1_address.as_slice().len()
            + self.l2_address.as_slice().len()
            + self.stake_info.as_slice().len()
            + self.checkpoint_type_id.as_slice().len()
            + self.xudt_type_id.as_slice().len()
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.amount.as_slice().len();
        offsets.push(total_size);
        total_size += self.version.as_slice().len();
        offsets.push(total_size);
        total_size += self.l1_address.as_slice().len();
        offsets.push(total_size);
        total_size += self.l2_address.as_slice().len();
        offsets.push(total_size);
        total_size += self.stake_info.as_slice().len();
        offsets.push(total_size);
        total_size += self.checkpoint_type_id.as_slice().len();
        offsets.push(total_size);
        total_size += self.xudt_type_id.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.amount.as_slice())?;
        writer.write_all(self.version.as_slice())?;
        writer.write_all(self.l1_address.as_slice())?;
        writer.write_all(self.l2_address.as_slice())?;
        writer.write_all(self.stake_info.as_slice())?;
        writer.write_all(self.checkpoint_type_id.as_slice())?;
        writer.write_all(self.xudt_type_id.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        StakeAtLockCellData::new_unchecked(inner.into())
    }
}
