use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct WriteOptions: u8 {
        const NONE = 0b0000_0000;
        const TIMESTAMP = 0b0000_0001;
        const LEVEL = 0b0000_0010;
        const FILE = 0b0000_0100;
        const LINE = 0b0000_1000;
        const MODULE = 0b0001_0000;
        const THREAD = 0b0010_0000;
        const BACKTRACE = 0b0100_0000;
        const LEVEL_COLOR = 0b1000_0000;

        const BASIC = Self::TIMESTAMP.bits() | Self::LEVEL.bits() | Self::LEVEL_COLOR.bits();
        const EXPANDED = Self::BASIC.bits() | Self::FILE.bits() | Self::LINE.bits() | Self::MODULE.bits() | Self::THREAD.bits();
        const ALL = !0;

        const BASIC_UNCOLORED = Self::BASIC.bits() & !Self::LEVEL_COLOR.bits();
        const EXPANDED_UNCOLORED = Self::EXPANDED.bits() & !Self::LEVEL_COLOR.bits();
        const ALL_UNCOLORED = Self::ALL.bits() & !Self::LEVEL_COLOR.bits();
    }
}

impl Default for WriteOptions {
    fn default() -> Self { Self::NONE }
}