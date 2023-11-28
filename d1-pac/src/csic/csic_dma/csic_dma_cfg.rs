#[doc = "Register `csic_dma_cfg` reader"]
pub type R = crate::R<CSIC_DMA_CFG_SPEC>;
#[doc = "Register `csic_dma_cfg` writer"]
pub type W = crate::W<CSIC_DMA_CFG_SPEC>;
#[doc = "Field `min_sdr_wr_size` reader - Minimum size of SDRAM block write"]
pub type MIN_SDR_WR_SIZE_R = crate::FieldReader<MIN_SDR_WR_SIZE_A>;
#[doc = "Minimum size of SDRAM block write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MIN_SDR_WR_SIZE_A {
    #[doc = "0: 256 bytes (if hflip is enabled, always select 256 bytes)"]
    _256 = 0,
    #[doc = "1: 512 bytes"]
    _512 = 1,
    #[doc = "2: 1K bytes"]
    _1K = 2,
    #[doc = "3: 2K bytes"]
    _2K = 3,
}
impl From<MIN_SDR_WR_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: MIN_SDR_WR_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MIN_SDR_WR_SIZE_A {
    type Ux = u8;
}
impl MIN_SDR_WR_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIN_SDR_WR_SIZE_A {
        match self.bits {
            0 => MIN_SDR_WR_SIZE_A::_256,
            1 => MIN_SDR_WR_SIZE_A::_512,
            2 => MIN_SDR_WR_SIZE_A::_1K,
            3 => MIN_SDR_WR_SIZE_A::_2K,
            _ => unreachable!(),
        }
    }
    #[doc = "256 bytes (if hflip is enabled, always select 256 bytes)"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == MIN_SDR_WR_SIZE_A::_256
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == MIN_SDR_WR_SIZE_A::_512
    }
    #[doc = "1K bytes"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        *self == MIN_SDR_WR_SIZE_A::_1K
    }
    #[doc = "2K bytes"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        *self == MIN_SDR_WR_SIZE_A::_2K
    }
}
#[doc = "Field `min_sdr_wr_size` writer - Minimum size of SDRAM block write"]
pub type MIN_SDR_WR_SIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MIN_SDR_WR_SIZE_A>;
impl<'a, REG> MIN_SDR_WR_SIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 bytes (if hflip is enabled, always select 256 bytes)"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(MIN_SDR_WR_SIZE_A::_256)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(MIN_SDR_WR_SIZE_A::_512)
    }
    #[doc = "1K bytes"]
    #[inline(always)]
    pub fn _1k(self) -> &'a mut crate::W<REG> {
        self.variant(MIN_SDR_WR_SIZE_A::_1K)
    }
    #[doc = "2K bytes"]
    #[inline(always)]
    pub fn _2k(self) -> &'a mut crate::W<REG> {
        self.variant(MIN_SDR_WR_SIZE_A::_2K)
    }
}
#[doc = "Field `fps_ds` reader - Fps down sample"]
pub type FPS_DS_R = crate::FieldReader<FPS_DS_A>;
#[doc = "Fps down sample\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FPS_DS_A {
    #[doc = "0: no down sample"]
    NO_DOWN_SAMPLE = 0,
    #[doc = "2: 1/3 fps, only receives the first frame every 3 frames"]
    _1_3 = 2,
    #[doc = "3: 1/4 fps, only receives the first frame every 4 frames"]
    _1_4 = 3,
    #[doc = "4: 1/5 fps, only receives the first frame every 5 frames"]
    _1_5 = 4,
    #[doc = "5: 1/6 fps, only receives the first frame every 6 frames"]
    _1_6 = 5,
    #[doc = "6: 1/7 fps, only receives the first frame every 7 frames"]
    _1_7 = 6,
    #[doc = "7: 1/8 fps, only receives the first frame every 8 frames"]
    _1_8 = 7,
    #[doc = "8: 1/9 fps, only receives the first frame every 9 frames"]
    _1_9 = 8,
    #[doc = "9: 1/10 fps, only receives the first frame every 10 frames"]
    _1_10 = 9,
    #[doc = "10: 1/11 fps, only receives the first frame every 11 frames"]
    _1_11 = 10,
    #[doc = "11: 1/12 fps, only receives the first frame every 12 frames"]
    _1_12 = 11,
    #[doc = "12: 1/13 fps, only receives the first frame every 13 frames"]
    _1_13 = 12,
    #[doc = "13: 1/14 fps, only receives the first frame every 14 frames"]
    _1_14 = 13,
    #[doc = "14: 1/15 fps, only receives the first frame every 15 frames"]
    _1_15 = 14,
    #[doc = "15: 1/16 fps, only receives the first frame every 16 frames"]
    _1_16 = 15,
}
impl From<FPS_DS_A> for u8 {
    #[inline(always)]
    fn from(variant: FPS_DS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FPS_DS_A {
    type Ux = u8;
}
impl FPS_DS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FPS_DS_A> {
        match self.bits {
            0 => Some(FPS_DS_A::NO_DOWN_SAMPLE),
            2 => Some(FPS_DS_A::_1_3),
            3 => Some(FPS_DS_A::_1_4),
            4 => Some(FPS_DS_A::_1_5),
            5 => Some(FPS_DS_A::_1_6),
            6 => Some(FPS_DS_A::_1_7),
            7 => Some(FPS_DS_A::_1_8),
            8 => Some(FPS_DS_A::_1_9),
            9 => Some(FPS_DS_A::_1_10),
            10 => Some(FPS_DS_A::_1_11),
            11 => Some(FPS_DS_A::_1_12),
            12 => Some(FPS_DS_A::_1_13),
            13 => Some(FPS_DS_A::_1_14),
            14 => Some(FPS_DS_A::_1_15),
            15 => Some(FPS_DS_A::_1_16),
            _ => None,
        }
    }
    #[doc = "no down sample"]
    #[inline(always)]
    pub fn is_no_down_sample(&self) -> bool {
        *self == FPS_DS_A::NO_DOWN_SAMPLE
    }
    #[doc = "1/3 fps, only receives the first frame every 3 frames"]
    #[inline(always)]
    pub fn is_1_3(&self) -> bool {
        *self == FPS_DS_A::_1_3
    }
    #[doc = "1/4 fps, only receives the first frame every 4 frames"]
    #[inline(always)]
    pub fn is_1_4(&self) -> bool {
        *self == FPS_DS_A::_1_4
    }
    #[doc = "1/5 fps, only receives the first frame every 5 frames"]
    #[inline(always)]
    pub fn is_1_5(&self) -> bool {
        *self == FPS_DS_A::_1_5
    }
    #[doc = "1/6 fps, only receives the first frame every 6 frames"]
    #[inline(always)]
    pub fn is_1_6(&self) -> bool {
        *self == FPS_DS_A::_1_6
    }
    #[doc = "1/7 fps, only receives the first frame every 7 frames"]
    #[inline(always)]
    pub fn is_1_7(&self) -> bool {
        *self == FPS_DS_A::_1_7
    }
    #[doc = "1/8 fps, only receives the first frame every 8 frames"]
    #[inline(always)]
    pub fn is_1_8(&self) -> bool {
        *self == FPS_DS_A::_1_8
    }
    #[doc = "1/9 fps, only receives the first frame every 9 frames"]
    #[inline(always)]
    pub fn is_1_9(&self) -> bool {
        *self == FPS_DS_A::_1_9
    }
    #[doc = "1/10 fps, only receives the first frame every 10 frames"]
    #[inline(always)]
    pub fn is_1_10(&self) -> bool {
        *self == FPS_DS_A::_1_10
    }
    #[doc = "1/11 fps, only receives the first frame every 11 frames"]
    #[inline(always)]
    pub fn is_1_11(&self) -> bool {
        *self == FPS_DS_A::_1_11
    }
    #[doc = "1/12 fps, only receives the first frame every 12 frames"]
    #[inline(always)]
    pub fn is_1_12(&self) -> bool {
        *self == FPS_DS_A::_1_12
    }
    #[doc = "1/13 fps, only receives the first frame every 13 frames"]
    #[inline(always)]
    pub fn is_1_13(&self) -> bool {
        *self == FPS_DS_A::_1_13
    }
    #[doc = "1/14 fps, only receives the first frame every 14 frames"]
    #[inline(always)]
    pub fn is_1_14(&self) -> bool {
        *self == FPS_DS_A::_1_14
    }
    #[doc = "1/15 fps, only receives the first frame every 15 frames"]
    #[inline(always)]
    pub fn is_1_15(&self) -> bool {
        *self == FPS_DS_A::_1_15
    }
    #[doc = "1/16 fps, only receives the first frame every 16 frames"]
    #[inline(always)]
    pub fn is_1_16(&self) -> bool {
        *self == FPS_DS_A::_1_16
    }
}
#[doc = "Field `fps_ds` writer - Fps down sample"]
pub type FPS_DS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, FPS_DS_A>;
impl<'a, REG> FPS_DS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no down sample"]
    #[inline(always)]
    pub fn no_down_sample(self) -> &'a mut crate::W<REG> {
        self.variant(FPS_DS_A::NO_DOWN_SAMPLE)
    }
    #[doc = "1/3 fps, only receives the first frame every 3 frames"]
    #[inline(always)]
    pub fn _1_3(self) -> &'a mut crate::W<REG> {
        self.variant(FPS_DS_A::_1_3)
    }
    #[doc = "1/4 fps, only receives the first frame every 4 frames"]
    #[inline(always)]
    pub fn _1_4(self) -> &'a mut crate::W<REG> {
        self.variant(FPS_DS_A::_1_4)
    }
    #[doc = "1/5 fps, only receives the first frame every 5 frames"]
    #[inline(always)]
    pub fn _1_5(self) -> &'a mut crate::W<REG> {
        self.variant(FPS_DS_A::_1_5)
    }
    #[doc = "1/6 fps, only receives the first frame every 6 frames"]
    #[inline(always)]
    pub fn _1_6(self) -> &'a mut crate::W<REG> {
        self.variant(FPS_DS_A::_1_6)
    }
    #[doc = "1/7 fps, only receives the first frame every 7 frames"]
    #[inline(always)]
    pub fn _1_7(self) -> &'a mut crate::W<REG> {
        self.variant(FPS_DS_A::_1_7)
    }
    #[doc = "1/8 fps, only receives the first frame every 8 frames"]
    #[inline(always)]
    pub fn _1_8(self) -> &'a mut crate::W<REG> {
        self.variant(FPS_DS_A::_1_8)
    }
    #[doc = "1/9 fps, only receives the first frame every 9 frames"]
    #[inline(always)]
    pub fn _1_9(self) -> &'a mut crate::W<REG> {
        self.variant(FPS_DS_A::_1_9)
    }
    #[doc = "1/10 fps, only receives the first frame every 10 frames"]
    #[inline(always)]
    pub fn _1_10(self) -> &'a mut crate::W<REG> {
        self.variant(FPS_DS_A::_1_10)
    }
    #[doc = "1/11 fps, only receives the first frame every 11 frames"]
    #[inline(always)]
    pub fn _1_11(self) -> &'a mut crate::W<REG> {
        self.variant(FPS_DS_A::_1_11)
    }
    #[doc = "1/12 fps, only receives the first frame every 12 frames"]
    #[inline(always)]
    pub fn _1_12(self) -> &'a mut crate::W<REG> {
        self.variant(FPS_DS_A::_1_12)
    }
    #[doc = "1/13 fps, only receives the first frame every 13 frames"]
    #[inline(always)]
    pub fn _1_13(self) -> &'a mut crate::W<REG> {
        self.variant(FPS_DS_A::_1_13)
    }
    #[doc = "1/14 fps, only receives the first frame every 14 frames"]
    #[inline(always)]
    pub fn _1_14(self) -> &'a mut crate::W<REG> {
        self.variant(FPS_DS_A::_1_14)
    }
    #[doc = "1/15 fps, only receives the first frame every 15 frames"]
    #[inline(always)]
    pub fn _1_15(self) -> &'a mut crate::W<REG> {
        self.variant(FPS_DS_A::_1_15)
    }
    #[doc = "1/16 fps, only receives the first frame every 16 frames"]
    #[inline(always)]
    pub fn _1_16(self) -> &'a mut crate::W<REG> {
        self.variant(FPS_DS_A::_1_16)
    }
}
#[doc = "Field `field_sel` reader - Field selection"]
pub type FIELD_SEL_R = crate::FieldReader<FIELD_SEL_A>;
#[doc = "Field selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FIELD_SEL_A {
    #[doc = "0: Capturing with field 0"]
    FIELD_0 = 0,
    #[doc = "1: Capturing with field 1"]
    FIELD_1 = 1,
    #[doc = "2: Capturing with either field"]
    EITHER_FIELD = 2,
}
impl From<FIELD_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FIELD_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FIELD_SEL_A {
    type Ux = u8;
}
impl FIELD_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FIELD_SEL_A> {
        match self.bits {
            0 => Some(FIELD_SEL_A::FIELD_0),
            1 => Some(FIELD_SEL_A::FIELD_1),
            2 => Some(FIELD_SEL_A::EITHER_FIELD),
            _ => None,
        }
    }
    #[doc = "Capturing with field 0"]
    #[inline(always)]
    pub fn is_field_0(&self) -> bool {
        *self == FIELD_SEL_A::FIELD_0
    }
    #[doc = "Capturing with field 1"]
    #[inline(always)]
    pub fn is_field_1(&self) -> bool {
        *self == FIELD_SEL_A::FIELD_1
    }
    #[doc = "Capturing with either field"]
    #[inline(always)]
    pub fn is_either_field(&self) -> bool {
        *self == FIELD_SEL_A::EITHER_FIELD
    }
}
#[doc = "Field `field_sel` writer - Field selection"]
pub type FIELD_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FIELD_SEL_A>;
impl<'a, REG> FIELD_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Capturing with field 0"]
    #[inline(always)]
    pub fn field_0(self) -> &'a mut crate::W<REG> {
        self.variant(FIELD_SEL_A::FIELD_0)
    }
    #[doc = "Capturing with field 1"]
    #[inline(always)]
    pub fn field_1(self) -> &'a mut crate::W<REG> {
        self.variant(FIELD_SEL_A::FIELD_1)
    }
    #[doc = "Capturing with either field"]
    #[inline(always)]
    pub fn either_field(self) -> &'a mut crate::W<REG> {
        self.variant(FIELD_SEL_A::EITHER_FIELD)
    }
}
#[doc = "Field `hflip_en` reader - Horizontal flip enable\n\nWhen enabled, the received data will be arranged in horizontal flip."]
pub type HFLIP_EN_R = crate::BitReader<HFLIP_EN_A>;
#[doc = "Horizontal flip enable\n\nWhen enabled, the received data will be arranged in horizontal flip.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFLIP_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<HFLIP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HFLIP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HFLIP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HFLIP_EN_A {
        match self.bits {
            false => HFLIP_EN_A::DISABLE,
            true => HFLIP_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HFLIP_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HFLIP_EN_A::ENABLE
    }
}
#[doc = "Field `hflip_en` writer - Horizontal flip enable\n\nWhen enabled, the received data will be arranged in horizontal flip."]
pub type HFLIP_EN_W<'a, REG> = crate::BitWriter<'a, REG, HFLIP_EN_A>;
impl<'a, REG> HFLIP_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HFLIP_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HFLIP_EN_A::ENABLE)
    }
}
#[doc = "Field `vflip_en` reader - Vertical flip enable\n\nWhen enabled, the received data will be arranged in vertical flip."]
pub type VFLIP_EN_R = crate::BitReader<VFLIP_EN_A>;
#[doc = "Vertical flip enable\n\nWhen enabled, the received data will be arranged in vertical flip.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VFLIP_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<VFLIP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VFLIP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl VFLIP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VFLIP_EN_A {
        match self.bits {
            false => VFLIP_EN_A::DISABLE,
            true => VFLIP_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VFLIP_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VFLIP_EN_A::ENABLE
    }
}
#[doc = "Field `vflip_en` writer - Vertical flip enable\n\nWhen enabled, the received data will be arranged in vertical flip."]
pub type VFLIP_EN_W<'a, REG> = crate::BitWriter<'a, REG, VFLIP_EN_A>;
impl<'a, REG> VFLIP_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(VFLIP_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(VFLIP_EN_A::ENABLE)
    }
}
#[doc = "Field `output_fmt` reader - Output data format\n\nWhen the input format is set to RAW stream\n\n0000: field-raw-8\n\n0001: field-raw-10\n\n0010: field-raw-12\n\n0011: reserved\n\n0100: field-rgb565\n\n0101: field-rgb888\n\n0110: field-prgb888\n\n0111: reserved\n\n1000: frame-raw-8\n\n1001: frame-raw-10\n\n1010: frame-raw-12\n\n1011: reserved\n\n1100: frame-rgb565\n\n1101: frame-rgb888\n\n1110: frame-prgb888\n\n1111: reserved\n\nWhen the input format is set to YUV422\n\n0000: field planar YCbCr 422\n\n0001: field planar YCbCr 420\n\n0010: frame planar YCbCr 420\n\n0011: frame planar YCbCr 422\n\n0100: field planar YCbCr 422 UV combined (UV sequence)\n\n0101: field planar YCbCr 420 UV combined (UV sequence)\n\n0110: frame planar YCbCr 420 UV combined (UV sequence)\n\n0111: frame planar YCbCr 422 UV combined (UV sequence)\n\n1000: filed planar YCbCr 422 UV combined (VU sequence)\n\n1001: field planar YCbCr 420 UV combined (VU sequence)\n\n1010: frame planar YCbCr 420 UV combined (VU sequence)\n\n1011: frame planar YCbCr 422 UV combined (VU sequence)\n\n1100: reserved\n\n1101: field YCbCr 400\n\n1110: reserved\n\n1111: frame YCbCr 400\n\nWhen the input format is set to YUV420\n\n0000: reserved\n\n0001: field planar YCbCr 420\n\n0010: frame planar YCbCr 420\n\n0011: reserved\n\n0100: reserved\n\n0101: field planar YCbCr 420 UV combined (UV sequence)\n\n0110: frame planar YCbCr 420 UV combined (UV sequence) 0111~1000: reserved\n\n1001: field planar YCbCr 420 UV combined (VU sequence)\n\n1010: frame planar YCbCr 420 UV combined (VU sequence) 1011~1100: reserved\n\n1101: field YCbCr 400\n\n1110: reserved\n\n1111: frame YCbCr 400"]
pub type OUTPUT_FMT_R = crate::FieldReader;
#[doc = "Field `output_fmt` writer - Output data format\n\nWhen the input format is set to RAW stream\n\n0000: field-raw-8\n\n0001: field-raw-10\n\n0010: field-raw-12\n\n0011: reserved\n\n0100: field-rgb565\n\n0101: field-rgb888\n\n0110: field-prgb888\n\n0111: reserved\n\n1000: frame-raw-8\n\n1001: frame-raw-10\n\n1010: frame-raw-12\n\n1011: reserved\n\n1100: frame-rgb565\n\n1101: frame-rgb888\n\n1110: frame-prgb888\n\n1111: reserved\n\nWhen the input format is set to YUV422\n\n0000: field planar YCbCr 422\n\n0001: field planar YCbCr 420\n\n0010: frame planar YCbCr 420\n\n0011: frame planar YCbCr 422\n\n0100: field planar YCbCr 422 UV combined (UV sequence)\n\n0101: field planar YCbCr 420 UV combined (UV sequence)\n\n0110: frame planar YCbCr 420 UV combined (UV sequence)\n\n0111: frame planar YCbCr 422 UV combined (UV sequence)\n\n1000: filed planar YCbCr 422 UV combined (VU sequence)\n\n1001: field planar YCbCr 420 UV combined (VU sequence)\n\n1010: frame planar YCbCr 420 UV combined (VU sequence)\n\n1011: frame planar YCbCr 422 UV combined (VU sequence)\n\n1100: reserved\n\n1101: field YCbCr 400\n\n1110: reserved\n\n1111: frame YCbCr 400\n\nWhen the input format is set to YUV420\n\n0000: reserved\n\n0001: field planar YCbCr 420\n\n0010: frame planar YCbCr 420\n\n0011: reserved\n\n0100: reserved\n\n0101: field planar YCbCr 420 UV combined (UV sequence)\n\n0110: frame planar YCbCr 420 UV combined (UV sequence) 0111~1000: reserved\n\n1001: field planar YCbCr 420 UV combined (VU sequence)\n\n1010: frame planar YCbCr 420 UV combined (VU sequence) 1011~1100: reserved\n\n1101: field YCbCr 400\n\n1110: reserved\n\n1111: frame YCbCr 400"]
pub type OUTPUT_FMT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `yuv_10bit_store_configuration` reader - 10-bit store configuration"]
pub type YUV_10BIT_STORE_CONFIGURATION_R = crate::BitReader<YUV_10BIT_STORE_CONFIGURATION_A>;
#[doc = "10-bit store configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YUV_10BIT_STORE_CONFIGURATION_A {
    #[doc = "0: YUV 10-bit stored in low 10-bit of a 16-bit word"]
    LOW = 0,
    #[doc = "1: YUV 10-bit stored in high 10-bit of a 16-bit word"]
    HIGH = 1,
}
impl From<YUV_10BIT_STORE_CONFIGURATION_A> for bool {
    #[inline(always)]
    fn from(variant: YUV_10BIT_STORE_CONFIGURATION_A) -> Self {
        variant as u8 != 0
    }
}
impl YUV_10BIT_STORE_CONFIGURATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> YUV_10BIT_STORE_CONFIGURATION_A {
        match self.bits {
            false => YUV_10BIT_STORE_CONFIGURATION_A::LOW,
            true => YUV_10BIT_STORE_CONFIGURATION_A::HIGH,
        }
    }
    #[doc = "YUV 10-bit stored in low 10-bit of a 16-bit word"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == YUV_10BIT_STORE_CONFIGURATION_A::LOW
    }
    #[doc = "YUV 10-bit stored in high 10-bit of a 16-bit word"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == YUV_10BIT_STORE_CONFIGURATION_A::HIGH
    }
}
#[doc = "Field `yuv_10bit_store_configuration` writer - 10-bit store configuration"]
pub type YUV_10BIT_STORE_CONFIGURATION_W<'a, REG> =
    crate::BitWriter<'a, REG, YUV_10BIT_STORE_CONFIGURATION_A>;
impl<'a, REG> YUV_10BIT_STORE_CONFIGURATION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "YUV 10-bit stored in low 10-bit of a 16-bit word"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(YUV_10BIT_STORE_CONFIGURATION_A::LOW)
    }
    #[doc = "YUV 10-bit stored in high 10-bit of a 16-bit word"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(YUV_10BIT_STORE_CONFIGURATION_A::HIGH)
    }
}
#[doc = "Field `yuv_10bit_cut_8bit` reader - 10-bit input cut to 8-bit"]
pub type YUV_10BIT_CUT_8BIT_R = crate::BitReader<YUV_10BIT_CUT_8BIT_A>;
#[doc = "10-bit input cut to 8-bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YUV_10BIT_CUT_8BIT_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<YUV_10BIT_CUT_8BIT_A> for bool {
    #[inline(always)]
    fn from(variant: YUV_10BIT_CUT_8BIT_A) -> Self {
        variant as u8 != 0
    }
}
impl YUV_10BIT_CUT_8BIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> YUV_10BIT_CUT_8BIT_A {
        match self.bits {
            false => YUV_10BIT_CUT_8BIT_A::DISABLE,
            true => YUV_10BIT_CUT_8BIT_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == YUV_10BIT_CUT_8BIT_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == YUV_10BIT_CUT_8BIT_A::ENABLE
    }
}
#[doc = "Field `yuv_10bit_cut_8bit` writer - 10-bit input cut to 8-bit"]
pub type YUV_10BIT_CUT_8BIT_W<'a, REG> = crate::BitWriter<'a, REG, YUV_10BIT_CUT_8BIT_A>;
impl<'a, REG> YUV_10BIT_CUT_8BIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(YUV_10BIT_CUT_8BIT_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(YUV_10BIT_CUT_8BIT_A::ENABLE)
    }
}
#[doc = "Field `pad_val` reader - Padding value when OUTPUT_FMT is prgb888\n\n0x00-0xff"]
pub type PAD_VAL_R = crate::FieldReader;
#[doc = "Field `pad_val` writer - Padding value when OUTPUT_FMT is prgb888\n\n0x00-0xff"]
pub type PAD_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Minimum size of SDRAM block write"]
    #[inline(always)]
    pub fn min_sdr_wr_size(&self) -> MIN_SDR_WR_SIZE_R {
        MIN_SDR_WR_SIZE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 6:9 - Fps down sample"]
    #[inline(always)]
    pub fn fps_ds(&self) -> FPS_DS_R {
        FPS_DS_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Field selection"]
    #[inline(always)]
    pub fn field_sel(&self) -> FIELD_SEL_R {
        FIELD_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Horizontal flip enable\n\nWhen enabled, the received data will be arranged in horizontal flip."]
    #[inline(always)]
    pub fn hflip_en(&self) -> HFLIP_EN_R {
        HFLIP_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Vertical flip enable\n\nWhen enabled, the received data will be arranged in vertical flip."]
    #[inline(always)]
    pub fn vflip_en(&self) -> VFLIP_EN_R {
        VFLIP_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Output data format\n\nWhen the input format is set to RAW stream\n\n0000: field-raw-8\n\n0001: field-raw-10\n\n0010: field-raw-12\n\n0011: reserved\n\n0100: field-rgb565\n\n0101: field-rgb888\n\n0110: field-prgb888\n\n0111: reserved\n\n1000: frame-raw-8\n\n1001: frame-raw-10\n\n1010: frame-raw-12\n\n1011: reserved\n\n1100: frame-rgb565\n\n1101: frame-rgb888\n\n1110: frame-prgb888\n\n1111: reserved\n\nWhen the input format is set to YUV422\n\n0000: field planar YCbCr 422\n\n0001: field planar YCbCr 420\n\n0010: frame planar YCbCr 420\n\n0011: frame planar YCbCr 422\n\n0100: field planar YCbCr 422 UV combined (UV sequence)\n\n0101: field planar YCbCr 420 UV combined (UV sequence)\n\n0110: frame planar YCbCr 420 UV combined (UV sequence)\n\n0111: frame planar YCbCr 422 UV combined (UV sequence)\n\n1000: filed planar YCbCr 422 UV combined (VU sequence)\n\n1001: field planar YCbCr 420 UV combined (VU sequence)\n\n1010: frame planar YCbCr 420 UV combined (VU sequence)\n\n1011: frame planar YCbCr 422 UV combined (VU sequence)\n\n1100: reserved\n\n1101: field YCbCr 400\n\n1110: reserved\n\n1111: frame YCbCr 400\n\nWhen the input format is set to YUV420\n\n0000: reserved\n\n0001: field planar YCbCr 420\n\n0010: frame planar YCbCr 420\n\n0011: reserved\n\n0100: reserved\n\n0101: field planar YCbCr 420 UV combined (UV sequence)\n\n0110: frame planar YCbCr 420 UV combined (UV sequence) 0111~1000: reserved\n\n1001: field planar YCbCr 420 UV combined (VU sequence)\n\n1010: frame planar YCbCr 420 UV combined (VU sequence) 1011~1100: reserved\n\n1101: field YCbCr 400\n\n1110: reserved\n\n1111: frame YCbCr 400"]
    #[inline(always)]
    pub fn output_fmt(&self) -> OUTPUT_FMT_R {
        OUTPUT_FMT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - 10-bit store configuration"]
    #[inline(always)]
    pub fn yuv_10bit_store_configuration(&self) -> YUV_10BIT_STORE_CONFIGURATION_R {
        YUV_10BIT_STORE_CONFIGURATION_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 10-bit input cut to 8-bit"]
    #[inline(always)]
    pub fn yuv_10bit_cut_8bit(&self) -> YUV_10BIT_CUT_8BIT_R {
        YUV_10BIT_CUT_8BIT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Padding value when OUTPUT_FMT is prgb888\n\n0x00-0xff"]
    #[inline(always)]
    pub fn pad_val(&self) -> PAD_VAL_R {
        PAD_VAL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Minimum size of SDRAM block write"]
    #[inline(always)]
    #[must_use]
    pub fn min_sdr_wr_size(&mut self) -> MIN_SDR_WR_SIZE_W<CSIC_DMA_CFG_SPEC> {
        MIN_SDR_WR_SIZE_W::new(self, 0)
    }
    #[doc = "Bits 6:9 - Fps down sample"]
    #[inline(always)]
    #[must_use]
    pub fn fps_ds(&mut self) -> FPS_DS_W<CSIC_DMA_CFG_SPEC> {
        FPS_DS_W::new(self, 6)
    }
    #[doc = "Bits 10:11 - Field selection"]
    #[inline(always)]
    #[must_use]
    pub fn field_sel(&mut self) -> FIELD_SEL_W<CSIC_DMA_CFG_SPEC> {
        FIELD_SEL_W::new(self, 10)
    }
    #[doc = "Bit 12 - Horizontal flip enable\n\nWhen enabled, the received data will be arranged in horizontal flip."]
    #[inline(always)]
    #[must_use]
    pub fn hflip_en(&mut self) -> HFLIP_EN_W<CSIC_DMA_CFG_SPEC> {
        HFLIP_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Vertical flip enable\n\nWhen enabled, the received data will be arranged in vertical flip."]
    #[inline(always)]
    #[must_use]
    pub fn vflip_en(&mut self) -> VFLIP_EN_W<CSIC_DMA_CFG_SPEC> {
        VFLIP_EN_W::new(self, 13)
    }
    #[doc = "Bits 16:19 - Output data format\n\nWhen the input format is set to RAW stream\n\n0000: field-raw-8\n\n0001: field-raw-10\n\n0010: field-raw-12\n\n0011: reserved\n\n0100: field-rgb565\n\n0101: field-rgb888\n\n0110: field-prgb888\n\n0111: reserved\n\n1000: frame-raw-8\n\n1001: frame-raw-10\n\n1010: frame-raw-12\n\n1011: reserved\n\n1100: frame-rgb565\n\n1101: frame-rgb888\n\n1110: frame-prgb888\n\n1111: reserved\n\nWhen the input format is set to YUV422\n\n0000: field planar YCbCr 422\n\n0001: field planar YCbCr 420\n\n0010: frame planar YCbCr 420\n\n0011: frame planar YCbCr 422\n\n0100: field planar YCbCr 422 UV combined (UV sequence)\n\n0101: field planar YCbCr 420 UV combined (UV sequence)\n\n0110: frame planar YCbCr 420 UV combined (UV sequence)\n\n0111: frame planar YCbCr 422 UV combined (UV sequence)\n\n1000: filed planar YCbCr 422 UV combined (VU sequence)\n\n1001: field planar YCbCr 420 UV combined (VU sequence)\n\n1010: frame planar YCbCr 420 UV combined (VU sequence)\n\n1011: frame planar YCbCr 422 UV combined (VU sequence)\n\n1100: reserved\n\n1101: field YCbCr 400\n\n1110: reserved\n\n1111: frame YCbCr 400\n\nWhen the input format is set to YUV420\n\n0000: reserved\n\n0001: field planar YCbCr 420\n\n0010: frame planar YCbCr 420\n\n0011: reserved\n\n0100: reserved\n\n0101: field planar YCbCr 420 UV combined (UV sequence)\n\n0110: frame planar YCbCr 420 UV combined (UV sequence) 0111~1000: reserved\n\n1001: field planar YCbCr 420 UV combined (VU sequence)\n\n1010: frame planar YCbCr 420 UV combined (VU sequence) 1011~1100: reserved\n\n1101: field YCbCr 400\n\n1110: reserved\n\n1111: frame YCbCr 400"]
    #[inline(always)]
    #[must_use]
    pub fn output_fmt(&mut self) -> OUTPUT_FMT_W<CSIC_DMA_CFG_SPEC> {
        OUTPUT_FMT_W::new(self, 16)
    }
    #[doc = "Bit 20 - 10-bit store configuration"]
    #[inline(always)]
    #[must_use]
    pub fn yuv_10bit_store_configuration(
        &mut self,
    ) -> YUV_10BIT_STORE_CONFIGURATION_W<CSIC_DMA_CFG_SPEC> {
        YUV_10BIT_STORE_CONFIGURATION_W::new(self, 20)
    }
    #[doc = "Bit 21 - 10-bit input cut to 8-bit"]
    #[inline(always)]
    #[must_use]
    pub fn yuv_10bit_cut_8bit(&mut self) -> YUV_10BIT_CUT_8BIT_W<CSIC_DMA_CFG_SPEC> {
        YUV_10BIT_CUT_8BIT_W::new(self, 21)
    }
    #[doc = "Bits 24:31 - Padding value when OUTPUT_FMT is prgb888\n\n0x00-0xff"]
    #[inline(always)]
    #[must_use]
    pub fn pad_val(&mut self) -> PAD_VAL_W<CSIC_DMA_CFG_SPEC> {
        PAD_VAL_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CSIC DMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_CFG_SPEC;
impl crate::RegisterSpec for CSIC_DMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_cfg::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_cfg::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_cfg to value 0"]
impl crate::Resettable for CSIC_DMA_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
