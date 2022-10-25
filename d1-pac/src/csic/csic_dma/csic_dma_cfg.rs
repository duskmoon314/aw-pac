#[doc = "Register `csic_dma_cfg` reader"]
pub struct R(crate::R<CSIC_DMA_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_cfg` writer"]
pub struct W(crate::W<CSIC_DMA_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CSIC_DMA_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `min_sdr_wr_size` reader - Minimum size of SDRAM block write"]
pub type MIN_SDR_WR_SIZE_R = crate::FieldReader<u8, MIN_SDR_WR_SIZE_A>;
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
impl MIN_SDR_WR_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIN_SDR_WR_SIZE_A {
        match self.bits {
            0 => MIN_SDR_WR_SIZE_A::_256,
            1 => MIN_SDR_WR_SIZE_A::_512,
            2 => MIN_SDR_WR_SIZE_A::_1K,
            3 => MIN_SDR_WR_SIZE_A::_2K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == MIN_SDR_WR_SIZE_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == MIN_SDR_WR_SIZE_A::_512
    }
    #[doc = "Checks if the value of the field is `_1K`"]
    #[inline(always)]
    pub fn is_1k(&self) -> bool {
        *self == MIN_SDR_WR_SIZE_A::_1K
    }
    #[doc = "Checks if the value of the field is `_2K`"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        *self == MIN_SDR_WR_SIZE_A::_2K
    }
}
#[doc = "Field `min_sdr_wr_size` writer - Minimum size of SDRAM block write"]
pub type MIN_SDR_WR_SIZE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CSIC_DMA_CFG_SPEC, u8, MIN_SDR_WR_SIZE_A, 2, O>;
impl<'a, const O: u8> MIN_SDR_WR_SIZE_W<'a, O> {
    #[doc = "256 bytes (if hflip is enabled, always select 256 bytes)"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(MIN_SDR_WR_SIZE_A::_256)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(MIN_SDR_WR_SIZE_A::_512)
    }
    #[doc = "1K bytes"]
    #[inline(always)]
    pub fn _1k(self) -> &'a mut W {
        self.variant(MIN_SDR_WR_SIZE_A::_1K)
    }
    #[doc = "2K bytes"]
    #[inline(always)]
    pub fn _2k(self) -> &'a mut W {
        self.variant(MIN_SDR_WR_SIZE_A::_2K)
    }
}
#[doc = "Field `fps_ds` reader - Fps down sample"]
pub type FPS_DS_R = crate::FieldReader<u8, FPS_DS_A>;
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
impl FPS_DS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FPS_DS_A> {
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
    #[doc = "Checks if the value of the field is `NO_DOWN_SAMPLE`"]
    #[inline(always)]
    pub fn is_no_down_sample(&self) -> bool {
        *self == FPS_DS_A::NO_DOWN_SAMPLE
    }
    #[doc = "Checks if the value of the field is `_1_3`"]
    #[inline(always)]
    pub fn is_1_3(&self) -> bool {
        *self == FPS_DS_A::_1_3
    }
    #[doc = "Checks if the value of the field is `_1_4`"]
    #[inline(always)]
    pub fn is_1_4(&self) -> bool {
        *self == FPS_DS_A::_1_4
    }
    #[doc = "Checks if the value of the field is `_1_5`"]
    #[inline(always)]
    pub fn is_1_5(&self) -> bool {
        *self == FPS_DS_A::_1_5
    }
    #[doc = "Checks if the value of the field is `_1_6`"]
    #[inline(always)]
    pub fn is_1_6(&self) -> bool {
        *self == FPS_DS_A::_1_6
    }
    #[doc = "Checks if the value of the field is `_1_7`"]
    #[inline(always)]
    pub fn is_1_7(&self) -> bool {
        *self == FPS_DS_A::_1_7
    }
    #[doc = "Checks if the value of the field is `_1_8`"]
    #[inline(always)]
    pub fn is_1_8(&self) -> bool {
        *self == FPS_DS_A::_1_8
    }
    #[doc = "Checks if the value of the field is `_1_9`"]
    #[inline(always)]
    pub fn is_1_9(&self) -> bool {
        *self == FPS_DS_A::_1_9
    }
    #[doc = "Checks if the value of the field is `_1_10`"]
    #[inline(always)]
    pub fn is_1_10(&self) -> bool {
        *self == FPS_DS_A::_1_10
    }
    #[doc = "Checks if the value of the field is `_1_11`"]
    #[inline(always)]
    pub fn is_1_11(&self) -> bool {
        *self == FPS_DS_A::_1_11
    }
    #[doc = "Checks if the value of the field is `_1_12`"]
    #[inline(always)]
    pub fn is_1_12(&self) -> bool {
        *self == FPS_DS_A::_1_12
    }
    #[doc = "Checks if the value of the field is `_1_13`"]
    #[inline(always)]
    pub fn is_1_13(&self) -> bool {
        *self == FPS_DS_A::_1_13
    }
    #[doc = "Checks if the value of the field is `_1_14`"]
    #[inline(always)]
    pub fn is_1_14(&self) -> bool {
        *self == FPS_DS_A::_1_14
    }
    #[doc = "Checks if the value of the field is `_1_15`"]
    #[inline(always)]
    pub fn is_1_15(&self) -> bool {
        *self == FPS_DS_A::_1_15
    }
    #[doc = "Checks if the value of the field is `_1_16`"]
    #[inline(always)]
    pub fn is_1_16(&self) -> bool {
        *self == FPS_DS_A::_1_16
    }
}
#[doc = "Field `fps_ds` writer - Fps down sample"]
pub type FPS_DS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_CFG_SPEC, u8, FPS_DS_A, 4, O>;
impl<'a, const O: u8> FPS_DS_W<'a, O> {
    #[doc = "no down sample"]
    #[inline(always)]
    pub fn no_down_sample(self) -> &'a mut W {
        self.variant(FPS_DS_A::NO_DOWN_SAMPLE)
    }
    #[doc = "1/3 fps, only receives the first frame every 3 frames"]
    #[inline(always)]
    pub fn _1_3(self) -> &'a mut W {
        self.variant(FPS_DS_A::_1_3)
    }
    #[doc = "1/4 fps, only receives the first frame every 4 frames"]
    #[inline(always)]
    pub fn _1_4(self) -> &'a mut W {
        self.variant(FPS_DS_A::_1_4)
    }
    #[doc = "1/5 fps, only receives the first frame every 5 frames"]
    #[inline(always)]
    pub fn _1_5(self) -> &'a mut W {
        self.variant(FPS_DS_A::_1_5)
    }
    #[doc = "1/6 fps, only receives the first frame every 6 frames"]
    #[inline(always)]
    pub fn _1_6(self) -> &'a mut W {
        self.variant(FPS_DS_A::_1_6)
    }
    #[doc = "1/7 fps, only receives the first frame every 7 frames"]
    #[inline(always)]
    pub fn _1_7(self) -> &'a mut W {
        self.variant(FPS_DS_A::_1_7)
    }
    #[doc = "1/8 fps, only receives the first frame every 8 frames"]
    #[inline(always)]
    pub fn _1_8(self) -> &'a mut W {
        self.variant(FPS_DS_A::_1_8)
    }
    #[doc = "1/9 fps, only receives the first frame every 9 frames"]
    #[inline(always)]
    pub fn _1_9(self) -> &'a mut W {
        self.variant(FPS_DS_A::_1_9)
    }
    #[doc = "1/10 fps, only receives the first frame every 10 frames"]
    #[inline(always)]
    pub fn _1_10(self) -> &'a mut W {
        self.variant(FPS_DS_A::_1_10)
    }
    #[doc = "1/11 fps, only receives the first frame every 11 frames"]
    #[inline(always)]
    pub fn _1_11(self) -> &'a mut W {
        self.variant(FPS_DS_A::_1_11)
    }
    #[doc = "1/12 fps, only receives the first frame every 12 frames"]
    #[inline(always)]
    pub fn _1_12(self) -> &'a mut W {
        self.variant(FPS_DS_A::_1_12)
    }
    #[doc = "1/13 fps, only receives the first frame every 13 frames"]
    #[inline(always)]
    pub fn _1_13(self) -> &'a mut W {
        self.variant(FPS_DS_A::_1_13)
    }
    #[doc = "1/14 fps, only receives the first frame every 14 frames"]
    #[inline(always)]
    pub fn _1_14(self) -> &'a mut W {
        self.variant(FPS_DS_A::_1_14)
    }
    #[doc = "1/15 fps, only receives the first frame every 15 frames"]
    #[inline(always)]
    pub fn _1_15(self) -> &'a mut W {
        self.variant(FPS_DS_A::_1_15)
    }
    #[doc = "1/16 fps, only receives the first frame every 16 frames"]
    #[inline(always)]
    pub fn _1_16(self) -> &'a mut W {
        self.variant(FPS_DS_A::_1_16)
    }
}
#[doc = "Field `field_sel` reader - Field selection"]
pub type FIELD_SEL_R = crate::FieldReader<u8, FIELD_SEL_A>;
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
impl FIELD_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FIELD_SEL_A> {
        match self.bits {
            0 => Some(FIELD_SEL_A::FIELD_0),
            1 => Some(FIELD_SEL_A::FIELD_1),
            2 => Some(FIELD_SEL_A::EITHER_FIELD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FIELD_0`"]
    #[inline(always)]
    pub fn is_field_0(&self) -> bool {
        *self == FIELD_SEL_A::FIELD_0
    }
    #[doc = "Checks if the value of the field is `FIELD_1`"]
    #[inline(always)]
    pub fn is_field_1(&self) -> bool {
        *self == FIELD_SEL_A::FIELD_1
    }
    #[doc = "Checks if the value of the field is `EITHER_FIELD`"]
    #[inline(always)]
    pub fn is_either_field(&self) -> bool {
        *self == FIELD_SEL_A::EITHER_FIELD
    }
}
#[doc = "Field `field_sel` writer - Field selection"]
pub type FIELD_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_CFG_SPEC, u8, FIELD_SEL_A, 2, O>;
impl<'a, const O: u8> FIELD_SEL_W<'a, O> {
    #[doc = "Capturing with field 0"]
    #[inline(always)]
    pub fn field_0(self) -> &'a mut W {
        self.variant(FIELD_SEL_A::FIELD_0)
    }
    #[doc = "Capturing with field 1"]
    #[inline(always)]
    pub fn field_1(self) -> &'a mut W {
        self.variant(FIELD_SEL_A::FIELD_1)
    }
    #[doc = "Capturing with either field"]
    #[inline(always)]
    pub fn either_field(self) -> &'a mut W {
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
    pub fn variant(&self) -> HFLIP_EN_A {
        match self.bits {
            false => HFLIP_EN_A::DISABLE,
            true => HFLIP_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HFLIP_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HFLIP_EN_A::ENABLE
    }
}
#[doc = "Field `hflip_en` writer - Horizontal flip enable\n\nWhen enabled, the received data will be arranged in horizontal flip."]
pub type HFLIP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIC_DMA_CFG_SPEC, HFLIP_EN_A, O>;
impl<'a, const O: u8> HFLIP_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HFLIP_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
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
    pub fn variant(&self) -> VFLIP_EN_A {
        match self.bits {
            false => VFLIP_EN_A::DISABLE,
            true => VFLIP_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VFLIP_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VFLIP_EN_A::ENABLE
    }
}
#[doc = "Field `vflip_en` writer - Vertical flip enable\n\nWhen enabled, the received data will be arranged in vertical flip."]
pub type VFLIP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIC_DMA_CFG_SPEC, VFLIP_EN_A, O>;
impl<'a, const O: u8> VFLIP_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VFLIP_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VFLIP_EN_A::ENABLE)
    }
}
#[doc = "Field `output_fmt` reader - Output data format\n\nWhen the input format is set to RAW stream\n\n0000: field-raw-8\n\n0001: field-raw-10\n\n0010: field-raw-12\n\n0011: reserved\n\n0100: field-rgb565\n\n0101: field-rgb888\n\n0110: field-prgb888\n\n0111: reserved\n\n1000: frame-raw-8\n\n1001: frame-raw-10\n\n1010: frame-raw-12\n\n1011: reserved\n\n1100: frame-rgb565\n\n1101: frame-rgb888\n\n1110: frame-prgb888\n\n1111: reserved\n\nWhen the input format is set to YUV422\n\n0000: field planar YCbCr 422\n\n0001: field planar YCbCr 420\n\n0010: frame planar YCbCr 420\n\n0011: frame planar YCbCr 422\n\n0100: field planar YCbCr 422 UV combined (UV sequence)\n\n0101: field planar YCbCr 420 UV combined (UV sequence)\n\n0110: frame planar YCbCr 420 UV combined (UV sequence)\n\n0111: frame planar YCbCr 422 UV combined (UV sequence)\n\n1000: filed planar YCbCr 422 UV combined (VU sequence)\n\n1001: field planar YCbCr 420 UV combined (VU sequence)\n\n1010: frame planar YCbCr 420 UV combined (VU sequence)\n\n1011: frame planar YCbCr 422 UV combined (VU sequence)\n\n1100: reserved\n\n1101: field YCbCr 400\n\n1110: reserved\n\n1111: frame YCbCr 400\n\nWhen the input format is set to YUV420\n\n0000: reserved\n\n0001: field planar YCbCr 420\n\n0010: frame planar YCbCr 420\n\n0011: reserved\n\n0100: reserved\n\n0101: field planar YCbCr 420 UV combined (UV sequence)\n\n0110: frame planar YCbCr 420 UV combined (UV sequence) 0111~1000: reserved\n\n1001: field planar YCbCr 420 UV combined (VU sequence)\n\n1010: frame planar YCbCr 420 UV combined (VU sequence) 1011~1100: reserved\n\n1101: field YCbCr 400\n\n1110: reserved\n\n1111: frame YCbCr 400"]
pub type OUTPUT_FMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `output_fmt` writer - Output data format\n\nWhen the input format is set to RAW stream\n\n0000: field-raw-8\n\n0001: field-raw-10\n\n0010: field-raw-12\n\n0011: reserved\n\n0100: field-rgb565\n\n0101: field-rgb888\n\n0110: field-prgb888\n\n0111: reserved\n\n1000: frame-raw-8\n\n1001: frame-raw-10\n\n1010: frame-raw-12\n\n1011: reserved\n\n1100: frame-rgb565\n\n1101: frame-rgb888\n\n1110: frame-prgb888\n\n1111: reserved\n\nWhen the input format is set to YUV422\n\n0000: field planar YCbCr 422\n\n0001: field planar YCbCr 420\n\n0010: frame planar YCbCr 420\n\n0011: frame planar YCbCr 422\n\n0100: field planar YCbCr 422 UV combined (UV sequence)\n\n0101: field planar YCbCr 420 UV combined (UV sequence)\n\n0110: frame planar YCbCr 420 UV combined (UV sequence)\n\n0111: frame planar YCbCr 422 UV combined (UV sequence)\n\n1000: filed planar YCbCr 422 UV combined (VU sequence)\n\n1001: field planar YCbCr 420 UV combined (VU sequence)\n\n1010: frame planar YCbCr 420 UV combined (VU sequence)\n\n1011: frame planar YCbCr 422 UV combined (VU sequence)\n\n1100: reserved\n\n1101: field YCbCr 400\n\n1110: reserved\n\n1111: frame YCbCr 400\n\nWhen the input format is set to YUV420\n\n0000: reserved\n\n0001: field planar YCbCr 420\n\n0010: frame planar YCbCr 420\n\n0011: reserved\n\n0100: reserved\n\n0101: field planar YCbCr 420 UV combined (UV sequence)\n\n0110: frame planar YCbCr 420 UV combined (UV sequence) 0111~1000: reserved\n\n1001: field planar YCbCr 420 UV combined (VU sequence)\n\n1010: frame planar YCbCr 420 UV combined (VU sequence) 1011~1100: reserved\n\n1101: field YCbCr 400\n\n1110: reserved\n\n1111: frame YCbCr 400"]
pub type OUTPUT_FMT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_CFG_SPEC, u8, u8, 4, O>;
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
    pub fn variant(&self) -> YUV_10BIT_STORE_CONFIGURATION_A {
        match self.bits {
            false => YUV_10BIT_STORE_CONFIGURATION_A::LOW,
            true => YUV_10BIT_STORE_CONFIGURATION_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == YUV_10BIT_STORE_CONFIGURATION_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == YUV_10BIT_STORE_CONFIGURATION_A::HIGH
    }
}
#[doc = "Field `yuv_10bit_store_configuration` writer - 10-bit store configuration"]
pub type YUV_10BIT_STORE_CONFIGURATION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSIC_DMA_CFG_SPEC, YUV_10BIT_STORE_CONFIGURATION_A, O>;
impl<'a, const O: u8> YUV_10BIT_STORE_CONFIGURATION_W<'a, O> {
    #[doc = "YUV 10-bit stored in low 10-bit of a 16-bit word"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(YUV_10BIT_STORE_CONFIGURATION_A::LOW)
    }
    #[doc = "YUV 10-bit stored in high 10-bit of a 16-bit word"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
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
    pub fn variant(&self) -> YUV_10BIT_CUT_8BIT_A {
        match self.bits {
            false => YUV_10BIT_CUT_8BIT_A::DISABLE,
            true => YUV_10BIT_CUT_8BIT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == YUV_10BIT_CUT_8BIT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == YUV_10BIT_CUT_8BIT_A::ENABLE
    }
}
#[doc = "Field `yuv_10bit_cut_8bit` writer - 10-bit input cut to 8-bit"]
pub type YUV_10BIT_CUT_8BIT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSIC_DMA_CFG_SPEC, YUV_10BIT_CUT_8BIT_A, O>;
impl<'a, const O: u8> YUV_10BIT_CUT_8BIT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(YUV_10BIT_CUT_8BIT_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(YUV_10BIT_CUT_8BIT_A::ENABLE)
    }
}
#[doc = "Field `pad_val` reader - Padding value when OUTPUT_FMT is prgb888\n\n0x00-0xff"]
pub type PAD_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_val` writer - Padding value when OUTPUT_FMT is prgb888\n\n0x00-0xff"]
pub type PAD_VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSIC_DMA_CFG_SPEC, u8, u8, 8, O>;
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
    pub fn min_sdr_wr_size(&mut self) -> MIN_SDR_WR_SIZE_W<0> {
        MIN_SDR_WR_SIZE_W::new(self)
    }
    #[doc = "Bits 6:9 - Fps down sample"]
    #[inline(always)]
    #[must_use]
    pub fn fps_ds(&mut self) -> FPS_DS_W<6> {
        FPS_DS_W::new(self)
    }
    #[doc = "Bits 10:11 - Field selection"]
    #[inline(always)]
    #[must_use]
    pub fn field_sel(&mut self) -> FIELD_SEL_W<10> {
        FIELD_SEL_W::new(self)
    }
    #[doc = "Bit 12 - Horizontal flip enable\n\nWhen enabled, the received data will be arranged in horizontal flip."]
    #[inline(always)]
    #[must_use]
    pub fn hflip_en(&mut self) -> HFLIP_EN_W<12> {
        HFLIP_EN_W::new(self)
    }
    #[doc = "Bit 13 - Vertical flip enable\n\nWhen enabled, the received data will be arranged in vertical flip."]
    #[inline(always)]
    #[must_use]
    pub fn vflip_en(&mut self) -> VFLIP_EN_W<13> {
        VFLIP_EN_W::new(self)
    }
    #[doc = "Bits 16:19 - Output data format\n\nWhen the input format is set to RAW stream\n\n0000: field-raw-8\n\n0001: field-raw-10\n\n0010: field-raw-12\n\n0011: reserved\n\n0100: field-rgb565\n\n0101: field-rgb888\n\n0110: field-prgb888\n\n0111: reserved\n\n1000: frame-raw-8\n\n1001: frame-raw-10\n\n1010: frame-raw-12\n\n1011: reserved\n\n1100: frame-rgb565\n\n1101: frame-rgb888\n\n1110: frame-prgb888\n\n1111: reserved\n\nWhen the input format is set to YUV422\n\n0000: field planar YCbCr 422\n\n0001: field planar YCbCr 420\n\n0010: frame planar YCbCr 420\n\n0011: frame planar YCbCr 422\n\n0100: field planar YCbCr 422 UV combined (UV sequence)\n\n0101: field planar YCbCr 420 UV combined (UV sequence)\n\n0110: frame planar YCbCr 420 UV combined (UV sequence)\n\n0111: frame planar YCbCr 422 UV combined (UV sequence)\n\n1000: filed planar YCbCr 422 UV combined (VU sequence)\n\n1001: field planar YCbCr 420 UV combined (VU sequence)\n\n1010: frame planar YCbCr 420 UV combined (VU sequence)\n\n1011: frame planar YCbCr 422 UV combined (VU sequence)\n\n1100: reserved\n\n1101: field YCbCr 400\n\n1110: reserved\n\n1111: frame YCbCr 400\n\nWhen the input format is set to YUV420\n\n0000: reserved\n\n0001: field planar YCbCr 420\n\n0010: frame planar YCbCr 420\n\n0011: reserved\n\n0100: reserved\n\n0101: field planar YCbCr 420 UV combined (UV sequence)\n\n0110: frame planar YCbCr 420 UV combined (UV sequence) 0111~1000: reserved\n\n1001: field planar YCbCr 420 UV combined (VU sequence)\n\n1010: frame planar YCbCr 420 UV combined (VU sequence) 1011~1100: reserved\n\n1101: field YCbCr 400\n\n1110: reserved\n\n1111: frame YCbCr 400"]
    #[inline(always)]
    #[must_use]
    pub fn output_fmt(&mut self) -> OUTPUT_FMT_W<16> {
        OUTPUT_FMT_W::new(self)
    }
    #[doc = "Bit 20 - 10-bit store configuration"]
    #[inline(always)]
    #[must_use]
    pub fn yuv_10bit_store_configuration(&mut self) -> YUV_10BIT_STORE_CONFIGURATION_W<20> {
        YUV_10BIT_STORE_CONFIGURATION_W::new(self)
    }
    #[doc = "Bit 21 - 10-bit input cut to 8-bit"]
    #[inline(always)]
    #[must_use]
    pub fn yuv_10bit_cut_8bit(&mut self) -> YUV_10BIT_CUT_8BIT_W<21> {
        YUV_10BIT_CUT_8BIT_W::new(self)
    }
    #[doc = "Bits 24:31 - Padding value when OUTPUT_FMT is prgb888\n\n0x00-0xff"]
    #[inline(always)]
    #[must_use]
    pub fn pad_val(&mut self) -> PAD_VAL_W<24> {
        PAD_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_cfg](index.html) module"]
pub struct CSIC_DMA_CFG_SPEC;
impl crate::RegisterSpec for CSIC_DMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_cfg::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_cfg::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_cfg to value 0"]
impl crate::Resettable for CSIC_DMA_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
