#[doc = "Register `prs_ncsic_if_cfg` reader"]
pub struct R(crate::R<PRS_NCSIC_IF_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRS_NCSIC_IF_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRS_NCSIC_IF_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRS_NCSIC_IF_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `prs_ncsic_if_cfg` writer"]
pub struct W(crate::W<PRS_NCSIC_IF_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRS_NCSIC_IF_CFG_SPEC>;
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
impl From<crate::W<PRS_NCSIC_IF_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRS_NCSIC_IF_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `csi_if` reader - "]
pub type CSI_IF_R = crate::FieldReader<u8, CSI_IF_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSI_IF_A {
    #[doc = "0: RAW or YUV420/YUYV422 (each cycle one component input)"]
    RAW_YUV420_YUYV422 = 0,
    #[doc = "4: BT656 1 channel"]
    BT656_1CH = 4,
    #[doc = "12: BT656 2 channel (All data interleaved in one data bus)"]
    BT656_2CH = 12,
    #[doc = "14: BT656 4 channel (All data interleaved in one data bus)"]
    BT656_4CH = 14,
}
impl From<CSI_IF_A> for u8 {
    #[inline(always)]
    fn from(variant: CSI_IF_A) -> Self {
        variant as _
    }
}
impl CSI_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSI_IF_A> {
        match self.bits {
            0 => Some(CSI_IF_A::RAW_YUV420_YUYV422),
            4 => Some(CSI_IF_A::BT656_1CH),
            12 => Some(CSI_IF_A::BT656_2CH),
            14 => Some(CSI_IF_A::BT656_4CH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RAW_YUV420_YUYV422`"]
    #[inline(always)]
    pub fn is_raw_yuv420_yuyv422(&self) -> bool {
        *self == CSI_IF_A::RAW_YUV420_YUYV422
    }
    #[doc = "Checks if the value of the field is `BT656_1CH`"]
    #[inline(always)]
    pub fn is_bt656_1ch(&self) -> bool {
        *self == CSI_IF_A::BT656_1CH
    }
    #[doc = "Checks if the value of the field is `BT656_2CH`"]
    #[inline(always)]
    pub fn is_bt656_2ch(&self) -> bool {
        *self == CSI_IF_A::BT656_2CH
    }
    #[doc = "Checks if the value of the field is `BT656_4CH`"]
    #[inline(always)]
    pub fn is_bt656_4ch(&self) -> bool {
        *self == CSI_IF_A::BT656_4CH
    }
}
#[doc = "Field `csi_if` writer - "]
pub type CSI_IF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRS_NCSIC_IF_CFG_SPEC, u8, CSI_IF_A, 5, O>;
impl<'a, const O: u8> CSI_IF_W<'a, O> {
    #[doc = "RAW or YUV420/YUYV422 (each cycle one component input)"]
    #[inline(always)]
    pub fn raw_yuv420_yuyv422(self) -> &'a mut W {
        self.variant(CSI_IF_A::RAW_YUV420_YUYV422)
    }
    #[doc = "BT656 1 channel"]
    #[inline(always)]
    pub fn bt656_1ch(self) -> &'a mut W {
        self.variant(CSI_IF_A::BT656_1CH)
    }
    #[doc = "BT656 2 channel (All data interleaved in one data bus)"]
    #[inline(always)]
    pub fn bt656_2ch(self) -> &'a mut W {
        self.variant(CSI_IF_A::BT656_2CH)
    }
    #[doc = "BT656 4 channel (All data interleaved in one data bus)"]
    #[inline(always)]
    pub fn bt656_4ch(self) -> &'a mut W {
        self.variant(CSI_IF_A::BT656_4CH)
    }
}
#[doc = "Field `input_seq` reader - Input data sequence, only valid for YUV422 and YUV420 input format"]
pub type INPUT_SEQ_R = crate::FieldReader<u8, INPUT_SEQ_A>;
#[doc = "Input data sequence, only valid for YUV422 and YUV420 input format\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUT_SEQ_A {
    #[doc = "0: `0`"]
    YUYV = 0,
    #[doc = "1: `1`"]
    YVYU = 1,
    #[doc = "2: `10`"]
    UYVY = 2,
    #[doc = "3: `11`"]
    VYUY = 3,
}
impl From<INPUT_SEQ_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT_SEQ_A) -> Self {
        variant as _
    }
}
impl INPUT_SEQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT_SEQ_A {
        match self.bits {
            0 => INPUT_SEQ_A::YUYV,
            1 => INPUT_SEQ_A::YVYU,
            2 => INPUT_SEQ_A::UYVY,
            3 => INPUT_SEQ_A::VYUY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `YUYV`"]
    #[inline(always)]
    pub fn is_yuyv(&self) -> bool {
        *self == INPUT_SEQ_A::YUYV
    }
    #[doc = "Checks if the value of the field is `YVYU`"]
    #[inline(always)]
    pub fn is_yvyu(&self) -> bool {
        *self == INPUT_SEQ_A::YVYU
    }
    #[doc = "Checks if the value of the field is `UYVY`"]
    #[inline(always)]
    pub fn is_uyvy(&self) -> bool {
        *self == INPUT_SEQ_A::UYVY
    }
    #[doc = "Checks if the value of the field is `VYUY`"]
    #[inline(always)]
    pub fn is_vyuy(&self) -> bool {
        *self == INPUT_SEQ_A::VYUY
    }
}
#[doc = "Field `input_seq` writer - Input data sequence, only valid for YUV422 and YUV420 input format"]
pub type INPUT_SEQ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRS_NCSIC_IF_CFG_SPEC, u8, INPUT_SEQ_A, 2, O>;
impl<'a, const O: u8> INPUT_SEQ_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn yuyv(self) -> &'a mut W {
        self.variant(INPUT_SEQ_A::YUYV)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn yvyu(self) -> &'a mut W {
        self.variant(INPUT_SEQ_A::YVYU)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn uyvy(self) -> &'a mut W {
        self.variant(INPUT_SEQ_A::UYVY)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn vyuy(self) -> &'a mut W {
        self.variant(INPUT_SEQ_A::VYUY)
    }
}
#[doc = "Field `if_data_width` reader - "]
pub type IF_DATA_WIDTH_R = crate::FieldReader<u8, IF_DATA_WIDTH_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IF_DATA_WIDTH_A {
    #[doc = "0: 8 bit data bus"]
    _8BIT = 0,
    #[doc = "1: 10 bit data bus"]
    _10BIT = 1,
    #[doc = "2: 12 bit data bus"]
    _12BIT = 2,
    #[doc = "3: 8 + 2 bit data bus"]
    _8P2BIT = 3,
    #[doc = "4: 2 * 8 bit data bus"]
    _2X8BIT = 4,
}
impl From<IF_DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: IF_DATA_WIDTH_A) -> Self {
        variant as _
    }
}
impl IF_DATA_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IF_DATA_WIDTH_A> {
        match self.bits {
            0 => Some(IF_DATA_WIDTH_A::_8BIT),
            1 => Some(IF_DATA_WIDTH_A::_10BIT),
            2 => Some(IF_DATA_WIDTH_A::_12BIT),
            3 => Some(IF_DATA_WIDTH_A::_8P2BIT),
            4 => Some(IF_DATA_WIDTH_A::_2X8BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == IF_DATA_WIDTH_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == IF_DATA_WIDTH_A::_10BIT
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == IF_DATA_WIDTH_A::_12BIT
    }
    #[doc = "Checks if the value of the field is `_8P2BIT`"]
    #[inline(always)]
    pub fn is_8p2bit(&self) -> bool {
        *self == IF_DATA_WIDTH_A::_8P2BIT
    }
    #[doc = "Checks if the value of the field is `_2X8BIT`"]
    #[inline(always)]
    pub fn is_2x8bit(&self) -> bool {
        *self == IF_DATA_WIDTH_A::_2X8BIT
    }
}
#[doc = "Field `if_data_width` writer - "]
pub type IF_DATA_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRS_NCSIC_IF_CFG_SPEC, u8, IF_DATA_WIDTH_A, 3, O>;
impl<'a, const O: u8> IF_DATA_WIDTH_W<'a, O> {
    #[doc = "8 bit data bus"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(IF_DATA_WIDTH_A::_8BIT)
    }
    #[doc = "10 bit data bus"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut W {
        self.variant(IF_DATA_WIDTH_A::_10BIT)
    }
    #[doc = "12 bit data bus"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(IF_DATA_WIDTH_A::_12BIT)
    }
    #[doc = "8 + 2 bit data bus"]
    #[inline(always)]
    pub fn _8p2bit(self) -> &'a mut W {
        self.variant(IF_DATA_WIDTH_A::_8P2BIT)
    }
    #[doc = "2 * 8 bit data bus"]
    #[inline(always)]
    pub fn _2x8bit(self) -> &'a mut W {
        self.variant(IF_DATA_WIDTH_A::_2X8BIT)
    }
}
#[doc = "Field `seq_8plus2` reader - When select IF_DATA_WIDTH to be 8+2bit, odd/even pixel byte at CSI-D\\[11:4\\] will be rearranged to D\\[11:2\\]+2'b0 at the actual CSI data bus according to these sequences"]
pub type SEQ_8PLUS2_R = crate::FieldReader<u8, SEQ_8PLUS2_A>;
#[doc = "When select IF_DATA_WIDTH to be 8+2bit, odd/even pixel byte at CSI-D\\[11:4\\] will be rearranged to D\\[11:2\\]+2'b0 at the actual CSI data bus according to these sequences\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEQ_8PLUS2_A {
    #[doc = "0: 6'bx + D\\[9:8\\], D\\[7:0\\]"]
    PD98_D70 = 0,
    #[doc = "1: D\\[9:2\\], 6'bx + D\\[1:0\\]"]
    D92_PD10 = 1,
    #[doc = "2: D\\[7:0\\], D\\[9:8\\] + 6'bx"]
    D70_D98P = 2,
    #[doc = "3: D\\[7:0\\], 6'bx + D\\[9:8\\]"]
    D70_PD98 = 3,
}
impl From<SEQ_8PLUS2_A> for u8 {
    #[inline(always)]
    fn from(variant: SEQ_8PLUS2_A) -> Self {
        variant as _
    }
}
impl SEQ_8PLUS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQ_8PLUS2_A {
        match self.bits {
            0 => SEQ_8PLUS2_A::PD98_D70,
            1 => SEQ_8PLUS2_A::D92_PD10,
            2 => SEQ_8PLUS2_A::D70_D98P,
            3 => SEQ_8PLUS2_A::D70_PD98,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PD98_D70`"]
    #[inline(always)]
    pub fn is_pd98_d70(&self) -> bool {
        *self == SEQ_8PLUS2_A::PD98_D70
    }
    #[doc = "Checks if the value of the field is `D92_PD10`"]
    #[inline(always)]
    pub fn is_d92_pd10(&self) -> bool {
        *self == SEQ_8PLUS2_A::D92_PD10
    }
    #[doc = "Checks if the value of the field is `D70_D98P`"]
    #[inline(always)]
    pub fn is_d70_d98p(&self) -> bool {
        *self == SEQ_8PLUS2_A::D70_D98P
    }
    #[doc = "Checks if the value of the field is `D70_PD98`"]
    #[inline(always)]
    pub fn is_d70_pd98(&self) -> bool {
        *self == SEQ_8PLUS2_A::D70_PD98
    }
}
#[doc = "Field `seq_8plus2` writer - When select IF_DATA_WIDTH to be 8+2bit, odd/even pixel byte at CSI-D\\[11:4\\] will be rearranged to D\\[11:2\\]+2'b0 at the actual CSI data bus according to these sequences"]
pub type SEQ_8PLUS2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRS_NCSIC_IF_CFG_SPEC, u8, SEQ_8PLUS2_A, 2, O>;
impl<'a, const O: u8> SEQ_8PLUS2_W<'a, O> {
    #[doc = "6'bx + D\\[9:8\\], D\\[7:0\\]"]
    #[inline(always)]
    pub fn pd98_d70(self) -> &'a mut W {
        self.variant(SEQ_8PLUS2_A::PD98_D70)
    }
    #[doc = "D\\[9:2\\], 6'bx + D\\[1:0\\]"]
    #[inline(always)]
    pub fn d92_pd10(self) -> &'a mut W {
        self.variant(SEQ_8PLUS2_A::D92_PD10)
    }
    #[doc = "D\\[7:0\\], D\\[9:8\\] + 6'bx"]
    #[inline(always)]
    pub fn d70_d98p(self) -> &'a mut W {
        self.variant(SEQ_8PLUS2_A::D70_D98P)
    }
    #[doc = "D\\[7:0\\], 6'bx + D\\[9:8\\]"]
    #[inline(always)]
    pub fn d70_pd98(self) -> &'a mut W {
        self.variant(SEQ_8PLUS2_A::D70_PD98)
    }
}
#[doc = "Field `ddr_sample_mode_en` reader - "]
pub type DDR_SAMPLE_MODE_EN_R = crate::BitReader<DDR_SAMPLE_MODE_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDR_SAMPLE_MODE_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<DDR_SAMPLE_MODE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DDR_SAMPLE_MODE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DDR_SAMPLE_MODE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDR_SAMPLE_MODE_EN_A {
        match self.bits {
            false => DDR_SAMPLE_MODE_EN_A::DISABLE,
            true => DDR_SAMPLE_MODE_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DDR_SAMPLE_MODE_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DDR_SAMPLE_MODE_EN_A::ENABLE
    }
}
#[doc = "Field `ddr_sample_mode_en` writer - "]
pub type DDR_SAMPLE_MODE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRS_NCSIC_IF_CFG_SPEC, DDR_SAMPLE_MODE_EN_A, O>;
impl<'a, const O: u8> DDR_SAMPLE_MODE_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DDR_SAMPLE_MODE_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DDR_SAMPLE_MODE_EN_A::ENABLE)
    }
}
#[doc = "Field `field_dt_mode` reader - only valid when CSI_IF is YUB and source type is interlaced"]
pub type FIELD_DT_MODE_R = crate::FieldReader<u8, FIELD_DT_MODE_A>;
#[doc = "only valid when CSI_IF is YUB and source type is interlaced\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FIELD_DT_MODE_A {
    #[doc = "0: by both field and vsync"]
    BY_FIELD_VSYNC = 0,
    #[doc = "1: by field"]
    BY_FIELD = 1,
    #[doc = "2: by vsync"]
    BY_VSYNC = 2,
}
impl From<FIELD_DT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FIELD_DT_MODE_A) -> Self {
        variant as _
    }
}
impl FIELD_DT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FIELD_DT_MODE_A> {
        match self.bits {
            0 => Some(FIELD_DT_MODE_A::BY_FIELD_VSYNC),
            1 => Some(FIELD_DT_MODE_A::BY_FIELD),
            2 => Some(FIELD_DT_MODE_A::BY_VSYNC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BY_FIELD_VSYNC`"]
    #[inline(always)]
    pub fn is_by_field_vsync(&self) -> bool {
        *self == FIELD_DT_MODE_A::BY_FIELD_VSYNC
    }
    #[doc = "Checks if the value of the field is `BY_FIELD`"]
    #[inline(always)]
    pub fn is_by_field(&self) -> bool {
        *self == FIELD_DT_MODE_A::BY_FIELD
    }
    #[doc = "Checks if the value of the field is `BY_VSYNC`"]
    #[inline(always)]
    pub fn is_by_vsync(&self) -> bool {
        *self == FIELD_DT_MODE_A::BY_VSYNC
    }
}
#[doc = "Field `field_dt_mode` writer - only valid when CSI_IF is YUB and source type is interlaced"]
pub type FIELD_DT_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRS_NCSIC_IF_CFG_SPEC, u8, FIELD_DT_MODE_A, 2, O>;
impl<'a, const O: u8> FIELD_DT_MODE_W<'a, O> {
    #[doc = "by both field and vsync"]
    #[inline(always)]
    pub fn by_field_vsync(self) -> &'a mut W {
        self.variant(FIELD_DT_MODE_A::BY_FIELD_VSYNC)
    }
    #[doc = "by field"]
    #[inline(always)]
    pub fn by_field(self) -> &'a mut W {
        self.variant(FIELD_DT_MODE_A::BY_FIELD)
    }
    #[doc = "by vsync"]
    #[inline(always)]
    pub fn by_vsync(self) -> &'a mut W {
        self.variant(FIELD_DT_MODE_A::BY_VSYNC)
    }
}
#[doc = "Field `clk_pol` reader - Data clock type"]
pub type CLK_POL_R = crate::BitReader<CLK_POL_A>;
#[doc = "Data clock type\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_POL_A {
    #[doc = "0: active in rising edge"]
    RISING = 0,
    #[doc = "1: active in falling edge"]
    FALLING = 1,
}
impl From<CLK_POL_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_POL_A {
        match self.bits {
            false => CLK_POL_A::RISING,
            true => CLK_POL_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == CLK_POL_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == CLK_POL_A::FALLING
    }
}
#[doc = "Field `clk_pol` writer - Data clock type"]
pub type CLK_POL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRS_NCSIC_IF_CFG_SPEC, CLK_POL_A, O>;
impl<'a, const O: u8> CLK_POL_W<'a, O> {
    #[doc = "active in rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(CLK_POL_A::RISING)
    }
    #[doc = "active in falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(CLK_POL_A::FALLING)
    }
}
#[doc = "Field `href_pol` reader - Href polarity\n\nThis register is not applied to CCIR656 interface"]
pub type HREF_POL_R = crate::BitReader<HREF_POL_A>;
#[doc = "Href polarity\n\nThis register is not applied to CCIR656 interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HREF_POL_A {
    #[doc = "0: `0`"]
    NEGATIVE = 0,
    #[doc = "1: `1`"]
    POSITIVE = 1,
}
impl From<HREF_POL_A> for bool {
    #[inline(always)]
    fn from(variant: HREF_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl HREF_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HREF_POL_A {
        match self.bits {
            false => HREF_POL_A::NEGATIVE,
            true => HREF_POL_A::POSITIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == HREF_POL_A::NEGATIVE
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == HREF_POL_A::POSITIVE
    }
}
#[doc = "Field `href_pol` writer - Href polarity\n\nThis register is not applied to CCIR656 interface"]
pub type HREF_POL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRS_NCSIC_IF_CFG_SPEC, HREF_POL_A, O>;
impl<'a, const O: u8> HREF_POL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(HREF_POL_A::NEGATIVE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(HREF_POL_A::POSITIVE)
    }
}
#[doc = "Field `vref_pol` reader - Vref polarity\n\nThis register is not applied to CCIR656 interface"]
pub type VREF_POL_R = crate::BitReader<VREF_POL_A>;
#[doc = "Vref polarity\n\nThis register is not applied to CCIR656 interface\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREF_POL_A {
    #[doc = "0: `0`"]
    NEGATIVE = 0,
    #[doc = "1: `1`"]
    POSITIVE = 1,
}
impl From<VREF_POL_A> for bool {
    #[inline(always)]
    fn from(variant: VREF_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl VREF_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREF_POL_A {
        match self.bits {
            false => VREF_POL_A::NEGATIVE,
            true => VREF_POL_A::POSITIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == VREF_POL_A::NEGATIVE
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == VREF_POL_A::POSITIVE
    }
}
#[doc = "Field `vref_pol` writer - Vref polarity\n\nThis register is not applied to CCIR656 interface"]
pub type VREF_POL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRS_NCSIC_IF_CFG_SPEC, VREF_POL_A, O>;
impl<'a, const O: u8> VREF_POL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(VREF_POL_A::NEGATIVE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(VREF_POL_A::POSITIVE)
    }
}
#[doc = "Field `field` reader - Field polarity (For YUV HV timing) / Field sequence (For BT656 timing)"]
pub type FIELD_R = crate::BitReader<FIELD_A>;
#[doc = "Field polarity (For YUV HV timing) / Field sequence (For BT656 timing)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIELD_A {
    #[doc = "0: `0`"]
    NEGATIVE_NORMAL_SEQ = 0,
    #[doc = "1: `1`"]
    POSITIVE_INVERSE_SEQ = 1,
}
impl From<FIELD_A> for bool {
    #[inline(always)]
    fn from(variant: FIELD_A) -> Self {
        variant as u8 != 0
    }
}
impl FIELD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELD_A {
        match self.bits {
            false => FIELD_A::NEGATIVE_NORMAL_SEQ,
            true => FIELD_A::POSITIVE_INVERSE_SEQ,
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_NORMAL_SEQ`"]
    #[inline(always)]
    pub fn is_negative_normal_seq(&self) -> bool {
        *self == FIELD_A::NEGATIVE_NORMAL_SEQ
    }
    #[doc = "Checks if the value of the field is `POSITIVE_INVERSE_SEQ`"]
    #[inline(always)]
    pub fn is_positive_inverse_seq(&self) -> bool {
        *self == FIELD_A::POSITIVE_INVERSE_SEQ
    }
}
#[doc = "Field `field` writer - Field polarity (For YUV HV timing) / Field sequence (For BT656 timing)"]
pub type FIELD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRS_NCSIC_IF_CFG_SPEC, FIELD_A, O>;
impl<'a, const O: u8> FIELD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_normal_seq(self) -> &'a mut W {
        self.variant(FIELD_A::NEGATIVE_NORMAL_SEQ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_inverse_seq(self) -> &'a mut W {
        self.variant(FIELD_A::POSITIVE_INVERSE_SEQ)
    }
}
#[doc = "Field `source_type` reader - Bit 20-23 corresponding to the SRC_TYPEs for channel 0-3"]
pub type SOURCE_TYPE_R = crate::FieldReader<u8, SOURCE_TYPE_A>;
#[doc = "Bit 20-23 corresponding to the SRC_TYPEs for channel 0-3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCE_TYPE_A {
    #[doc = "0: `0`"]
    P_ROGRESSED = 0,
    #[doc = "1: `1`"]
    INTERLACED = 1,
}
impl From<SOURCE_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCE_TYPE_A) -> Self {
        variant as _
    }
}
impl SOURCE_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SOURCE_TYPE_A> {
        match self.bits {
            0 => Some(SOURCE_TYPE_A::P_ROGRESSED),
            1 => Some(SOURCE_TYPE_A::INTERLACED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P_ROGRESSED`"]
    #[inline(always)]
    pub fn is_p_rogressed(&self) -> bool {
        *self == SOURCE_TYPE_A::P_ROGRESSED
    }
    #[doc = "Checks if the value of the field is `INTERLACED`"]
    #[inline(always)]
    pub fn is_interlaced(&self) -> bool {
        *self == SOURCE_TYPE_A::INTERLACED
    }
}
#[doc = "Field `source_type` writer - Bit 20-23 corresponding to the SRC_TYPEs for channel 0-3"]
pub type SOURCE_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRS_NCSIC_IF_CFG_SPEC, u8, SOURCE_TYPE_A, 4, O>;
impl<'a, const O: u8> SOURCE_TYPE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn p_rogressed(self) -> &'a mut W {
        self.variant(SOURCE_TYPE_A::P_ROGRESSED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn interlaced(self) -> &'a mut W {
        self.variant(SOURCE_TYPE_A::INTERLACED)
    }
}
#[doc = "Field `field_dt_pclk_shift` reader - Only for vsync detected field mode, the odd field permitted pclk\n\nshift = 4 * FIELD_DT_PCLK_SHIFT"]
pub type FIELD_DT_PCLK_SHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `field_dt_pclk_shift` writer - Only for vsync detected field mode, the odd field permitted pclk\n\nshift = 4 * FIELD_DT_PCLK_SHIFT"]
pub type FIELD_DT_PCLK_SHIFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRS_NCSIC_IF_CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `yuv420_line_order` reader - "]
pub type YUV420_LINE_ORDER_R = crate::BitReader<YUV420_LINE_ORDER_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YUV420_LINE_ORDER_A {
    #[doc = "0: YUV420 input in Y_YC_Y_YC line order"]
    Y_YC_Y_YC = 0,
    #[doc = "1: YUV420 input in YC_Y_YC_Y line order"]
    YC_Y_YC_Y = 1,
}
impl From<YUV420_LINE_ORDER_A> for bool {
    #[inline(always)]
    fn from(variant: YUV420_LINE_ORDER_A) -> Self {
        variant as u8 != 0
    }
}
impl YUV420_LINE_ORDER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YUV420_LINE_ORDER_A {
        match self.bits {
            false => YUV420_LINE_ORDER_A::Y_YC_Y_YC,
            true => YUV420_LINE_ORDER_A::YC_Y_YC_Y,
        }
    }
    #[doc = "Checks if the value of the field is `Y_YC_Y_YC`"]
    #[inline(always)]
    pub fn is_y_yc_y_yc(&self) -> bool {
        *self == YUV420_LINE_ORDER_A::Y_YC_Y_YC
    }
    #[doc = "Checks if the value of the field is `YC_Y_YC_Y`"]
    #[inline(always)]
    pub fn is_yc_y_yc_y(&self) -> bool {
        *self == YUV420_LINE_ORDER_A::YC_Y_YC_Y
    }
}
#[doc = "Field `yuv420_line_order` writer - "]
pub type YUV420_LINE_ORDER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRS_NCSIC_IF_CFG_SPEC, YUV420_LINE_ORDER_A, O>;
impl<'a, const O: u8> YUV420_LINE_ORDER_W<'a, O> {
    #[doc = "YUV420 input in Y_YC_Y_YC line order"]
    #[inline(always)]
    pub fn y_yc_y_yc(self) -> &'a mut W {
        self.variant(YUV420_LINE_ORDER_A::Y_YC_Y_YC)
    }
    #[doc = "YUV420 input in YC_Y_YC_Y line order"]
    #[inline(always)]
    pub fn yc_y_yc_y(self) -> &'a mut W {
        self.variant(YUV420_LINE_ORDER_A::YC_Y_YC_Y)
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn csi_if(&self) -> CSI_IF_R {
        CSI_IF_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - Input data sequence, only valid for YUV422 and YUV420 input format"]
    #[inline(always)]
    pub fn input_seq(&self) -> INPUT_SEQ_R {
        INPUT_SEQ_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn if_data_width(&self) -> IF_DATA_WIDTH_R {
        IF_DATA_WIDTH_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - When select IF_DATA_WIDTH to be 8+2bit, odd/even pixel byte at CSI-D\\[11:4\\] will be rearranged to D\\[11:2\\]+2'b0 at the actual CSI data bus according to these sequences"]
    #[inline(always)]
    pub fn seq_8plus2(&self) -> SEQ_8PLUS2_R {
        SEQ_8PLUS2_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ddr_sample_mode_en(&self) -> DDR_SAMPLE_MODE_EN_R {
        DDR_SAMPLE_MODE_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - only valid when CSI_IF is YUB and source type is interlaced"]
    #[inline(always)]
    pub fn field_dt_mode(&self) -> FIELD_DT_MODE_R {
        FIELD_DT_MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Data clock type"]
    #[inline(always)]
    pub fn clk_pol(&self) -> CLK_POL_R {
        CLK_POL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Href polarity\n\nThis register is not applied to CCIR656 interface"]
    #[inline(always)]
    pub fn href_pol(&self) -> HREF_POL_R {
        HREF_POL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Vref polarity\n\nThis register is not applied to CCIR656 interface"]
    #[inline(always)]
    pub fn vref_pol(&self) -> VREF_POL_R {
        VREF_POL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Field polarity (For YUV HV timing) / Field sequence (For BT656 timing)"]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Bit 20-23 corresponding to the SRC_TYPEs for channel 0-3"]
    #[inline(always)]
    pub fn source_type(&self) -> SOURCE_TYPE_R {
        SOURCE_TYPE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Only for vsync detected field mode, the odd field permitted pclk\n\nshift = 4 * FIELD_DT_PCLK_SHIFT"]
    #[inline(always)]
    pub fn field_dt_pclk_shift(&self) -> FIELD_DT_PCLK_SHIFT_R {
        FIELD_DT_PCLK_SHIFT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn yuv420_line_order(&self) -> YUV420_LINE_ORDER_R {
        YUV420_LINE_ORDER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn csi_if(&mut self) -> CSI_IF_W<0> {
        CSI_IF_W::new(self)
    }
    #[doc = "Bits 6:7 - Input data sequence, only valid for YUV422 and YUV420 input format"]
    #[inline(always)]
    #[must_use]
    pub fn input_seq(&mut self) -> INPUT_SEQ_W<6> {
        INPUT_SEQ_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn if_data_width(&mut self) -> IF_DATA_WIDTH_W<8> {
        IF_DATA_WIDTH_W::new(self)
    }
    #[doc = "Bits 11:12 - When select IF_DATA_WIDTH to be 8+2bit, odd/even pixel byte at CSI-D\\[11:4\\] will be rearranged to D\\[11:2\\]+2'b0 at the actual CSI data bus according to these sequences"]
    #[inline(always)]
    #[must_use]
    pub fn seq_8plus2(&mut self) -> SEQ_8PLUS2_W<11> {
        SEQ_8PLUS2_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ddr_sample_mode_en(&mut self) -> DDR_SAMPLE_MODE_EN_W<13> {
        DDR_SAMPLE_MODE_EN_W::new(self)
    }
    #[doc = "Bits 14:15 - only valid when CSI_IF is YUB and source type is interlaced"]
    #[inline(always)]
    #[must_use]
    pub fn field_dt_mode(&mut self) -> FIELD_DT_MODE_W<14> {
        FIELD_DT_MODE_W::new(self)
    }
    #[doc = "Bit 16 - Data clock type"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pol(&mut self) -> CLK_POL_W<16> {
        CLK_POL_W::new(self)
    }
    #[doc = "Bit 17 - Href polarity\n\nThis register is not applied to CCIR656 interface"]
    #[inline(always)]
    #[must_use]
    pub fn href_pol(&mut self) -> HREF_POL_W<17> {
        HREF_POL_W::new(self)
    }
    #[doc = "Bit 18 - Vref polarity\n\nThis register is not applied to CCIR656 interface"]
    #[inline(always)]
    #[must_use]
    pub fn vref_pol(&mut self) -> VREF_POL_W<18> {
        VREF_POL_W::new(self)
    }
    #[doc = "Bit 19 - Field polarity (For YUV HV timing) / Field sequence (For BT656 timing)"]
    #[inline(always)]
    #[must_use]
    pub fn field(&mut self) -> FIELD_W<19> {
        FIELD_W::new(self)
    }
    #[doc = "Bits 20:23 - Bit 20-23 corresponding to the SRC_TYPEs for channel 0-3"]
    #[inline(always)]
    #[must_use]
    pub fn source_type(&mut self) -> SOURCE_TYPE_W<20> {
        SOURCE_TYPE_W::new(self)
    }
    #[doc = "Bits 24:27 - Only for vsync detected field mode, the odd field permitted pclk\n\nshift = 4 * FIELD_DT_PCLK_SHIFT"]
    #[inline(always)]
    #[must_use]
    pub fn field_dt_pclk_shift(&mut self) -> FIELD_DT_PCLK_SHIFT_W<24> {
        FIELD_DT_PCLK_SHIFT_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn yuv420_line_order(&mut self) -> YUV420_LINE_ORDER_W<31> {
        YUV420_LINE_ORDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parser NCSIC Interface Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prs_ncsic_if_cfg](index.html) module"]
pub struct PRS_NCSIC_IF_CFG_SPEC;
impl crate::RegisterSpec for PRS_NCSIC_IF_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prs_ncsic_if_cfg::R](R) reader structure"]
impl crate::Readable for PRS_NCSIC_IF_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prs_ncsic_if_cfg::W](W) writer structure"]
impl crate::Writable for PRS_NCSIC_IF_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets prs_ncsic_if_cfg to value 0x0105_0080"]
impl crate::Resettable for PRS_NCSIC_IF_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0105_0080;
}
