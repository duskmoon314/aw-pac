#[doc = "Register `tve_configuration` reader"]
pub type R = crate::R<TVE_CONFIGURATION_SPEC>;
#[doc = "Register `tve_configuration` writer"]
pub type W = crate::W<TVE_CONFIGURATION_SPEC>;
#[doc = "Field `tvmode_select` reader - TVMode_Select\n\nNote: Changing this register value will cause some relative register setting to relative value."]
pub type TVMODE_SELECT_R = crate::FieldReader<TVMODE_SELECT_A>;
#[doc = "TVMode_Select\n\nNote: Changing this register value will cause some relative register setting to relative value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TVMODE_SELECT_A {
    #[doc = "0: NTSC"]
    NTSC = 0,
    #[doc = "1: PAL"]
    PAL = 1,
}
impl From<TVMODE_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TVMODE_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TVMODE_SELECT_A {
    type Ux = u8;
}
impl TVMODE_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TVMODE_SELECT_A> {
        match self.bits {
            0 => Some(TVMODE_SELECT_A::NTSC),
            1 => Some(TVMODE_SELECT_A::PAL),
            _ => None,
        }
    }
    #[doc = "NTSC"]
    #[inline(always)]
    pub fn is_ntsc(&self) -> bool {
        *self == TVMODE_SELECT_A::NTSC
    }
    #[doc = "PAL"]
    #[inline(always)]
    pub fn is_pal(&self) -> bool {
        *self == TVMODE_SELECT_A::PAL
    }
}
#[doc = "Field `tvmode_select` writer - TVMode_Select\n\nNote: Changing this register value will cause some relative register setting to relative value."]
pub type TVMODE_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TVMODE_SELECT_A>;
impl<'a, REG> TVMODE_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NTSC"]
    #[inline(always)]
    pub fn ntsc(self) -> &'a mut crate::W<REG> {
        self.variant(TVMODE_SELECT_A::NTSC)
    }
    #[doc = "PAL"]
    #[inline(always)]
    pub fn pal(self) -> &'a mut crate::W<REG> {
        self.variant(TVMODE_SELECT_A::PAL)
    }
}
#[doc = "Field `mode_1080i_1250line_sel` reader - Mode_1080i_1250Line_Sel"]
pub type MODE_1080I_1250LINE_SEL_R = crate::BitReader<MODE_1080I_1250LINE_SEL_A>;
#[doc = "Mode_1080i_1250Line_Sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_1080I_1250LINE_SEL_A {
    #[doc = "0: 1125 Line mode"]
    _1125 = 0,
    #[doc = "1: 1250 Line mode"]
    _1250 = 1,
}
impl From<MODE_1080I_1250LINE_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_1080I_1250LINE_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_1080I_1250LINE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE_1080I_1250LINE_SEL_A {
        match self.bits {
            false => MODE_1080I_1250LINE_SEL_A::_1125,
            true => MODE_1080I_1250LINE_SEL_A::_1250,
        }
    }
    #[doc = "1125 Line mode"]
    #[inline(always)]
    pub fn is_1125(&self) -> bool {
        *self == MODE_1080I_1250LINE_SEL_A::_1125
    }
    #[doc = "1250 Line mode"]
    #[inline(always)]
    pub fn is_1250(&self) -> bool {
        *self == MODE_1080I_1250LINE_SEL_A::_1250
    }
}
#[doc = "Field `mode_1080i_1250line_sel` writer - Mode_1080i_1250Line_Sel"]
pub type MODE_1080I_1250LINE_SEL_W<'a, REG> = crate::BitWriter<'a, REG, MODE_1080I_1250LINE_SEL_A>;
impl<'a, REG> MODE_1080I_1250LINE_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1125 Line mode"]
    #[inline(always)]
    pub fn _1125(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_1080I_1250LINE_SEL_A::_1125)
    }
    #[doc = "1250 Line mode"]
    #[inline(always)]
    pub fn _1250(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_1080I_1250LINE_SEL_A::_1250)
    }
}
#[doc = "Field `color_bar_mode` reader - Standard Color bar input selection\n\nThis bit selects whether the Video Encoder video data input is replaced by an internal standard color bar generator or not."]
pub type COLOR_BAR_MODE_R = crate::BitReader<COLOR_BAR_MODE_A>;
#[doc = "Standard Color bar input selection\n\nThis bit selects whether the Video Encoder video data input is replaced by an internal standard color bar generator or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLOR_BAR_MODE_A {
    #[doc = "0: The Video Encoder input is coming from the Display Engineer"]
    D_ISPLAY_E_NGINEER = 0,
    #[doc = "1: The Video Encoder input is coming from an internal standard color bar generator."]
    I_NTERNAL_GENERATOR = 1,
}
impl From<COLOR_BAR_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: COLOR_BAR_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl COLOR_BAR_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COLOR_BAR_MODE_A {
        match self.bits {
            false => COLOR_BAR_MODE_A::D_ISPLAY_E_NGINEER,
            true => COLOR_BAR_MODE_A::I_NTERNAL_GENERATOR,
        }
    }
    #[doc = "The Video Encoder input is coming from the Display Engineer"]
    #[inline(always)]
    pub fn is_d_isplay_e_ngineer(&self) -> bool {
        *self == COLOR_BAR_MODE_A::D_ISPLAY_E_NGINEER
    }
    #[doc = "The Video Encoder input is coming from an internal standard color bar generator."]
    #[inline(always)]
    pub fn is_i_nternal_generator(&self) -> bool {
        *self == COLOR_BAR_MODE_A::I_NTERNAL_GENERATOR
    }
}
#[doc = "Field `color_bar_mode` writer - Standard Color bar input selection\n\nThis bit selects whether the Video Encoder video data input is replaced by an internal standard color bar generator or not."]
pub type COLOR_BAR_MODE_W<'a, REG> = crate::BitWriter<'a, REG, COLOR_BAR_MODE_A>;
impl<'a, REG> COLOR_BAR_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Video Encoder input is coming from the Display Engineer"]
    #[inline(always)]
    pub fn d_isplay_e_ngineer(self) -> &'a mut crate::W<REG> {
        self.variant(COLOR_BAR_MODE_A::D_ISPLAY_E_NGINEER)
    }
    #[doc = "The Video Encoder input is coming from an internal standard color bar generator."]
    #[inline(always)]
    pub fn i_nternal_generator(self) -> &'a mut crate::W<REG> {
        self.variant(COLOR_BAR_MODE_A::I_NTERNAL_GENERATOR)
    }
}
#[doc = "Field `color_bar_type` reader - "]
pub type COLOR_BAR_TYPE_R = crate::BitReader<COLOR_BAR_TYPE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLOR_BAR_TYPE_A {
    #[doc = "0: 75/7.5/75/7.5 (NTSC), 100/0/75/0(PAL)"]
    _75 = 0,
    #[doc = "1: 100/7.5/100/7.5(NTSC), 100/0/100/0(PAL)"]
    _100 = 1,
}
impl From<COLOR_BAR_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: COLOR_BAR_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl COLOR_BAR_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COLOR_BAR_TYPE_A {
        match self.bits {
            false => COLOR_BAR_TYPE_A::_75,
            true => COLOR_BAR_TYPE_A::_100,
        }
    }
    #[doc = "75/7.5/75/7.5 (NTSC), 100/0/75/0(PAL)"]
    #[inline(always)]
    pub fn is_75(&self) -> bool {
        *self == COLOR_BAR_TYPE_A::_75
    }
    #[doc = "100/7.5/100/7.5(NTSC), 100/0/100/0(PAL)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == COLOR_BAR_TYPE_A::_100
    }
}
#[doc = "Field `color_bar_type` writer - "]
pub type COLOR_BAR_TYPE_W<'a, REG> = crate::BitWriter<'a, REG, COLOR_BAR_TYPE_A>;
impl<'a, REG> COLOR_BAR_TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "75/7.5/75/7.5 (NTSC), 100/0/75/0(PAL)"]
    #[inline(always)]
    pub fn _75(self) -> &'a mut crate::W<REG> {
        self.variant(COLOR_BAR_TYPE_A::_75)
    }
    #[doc = "100/7.5/100/7.5(NTSC), 100/0/100/0(PAL)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(COLOR_BAR_TYPE_A::_100)
    }
}
#[doc = "Field `cvbs_en` reader - Composite video enables selection\n\nThis bit selects whether the composite video output (CVBS) is enabled or disabled."]
pub type CVBS_EN_R = crate::BitReader<CVBS_EN_A>;
#[doc = "Composite video enables selection\n\nThis bit selects whether the composite video output (CVBS) is enabled or disabled.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CVBS_EN_A {
    #[doc = "0: Composite video is disabled, Only Y/C is enabled"]
    DISABLED = 0,
    #[doc = "1: Composite video is enabled., CVBS and Y/C are enabled"]
    ENABLED = 1,
}
impl From<CVBS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CVBS_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CVBS_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CVBS_EN_A {
        match self.bits {
            false => CVBS_EN_A::DISABLED,
            true => CVBS_EN_A::ENABLED,
        }
    }
    #[doc = "Composite video is disabled, Only Y/C is enabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CVBS_EN_A::DISABLED
    }
    #[doc = "Composite video is enabled., CVBS and Y/C are enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CVBS_EN_A::ENABLED
    }
}
#[doc = "Field `cvbs_en` writer - Composite video enables selection\n\nThis bit selects whether the composite video output (CVBS) is enabled or disabled."]
pub type CVBS_EN_W<'a, REG> = crate::BitWriter<'a, REG, CVBS_EN_A>;
impl<'a, REG> CVBS_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Composite video is disabled, Only Y/C is enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CVBS_EN_A::DISABLED)
    }
    #[doc = "Composite video is enabled., CVBS and Y/C are enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CVBS_EN_A::ENABLED)
    }
}
#[doc = "Field `yc_en` reader - S-port Video enable Selection.\n\nThis bit selects whether the S-port(Y/C) video output is enabled or disabled."]
pub type YC_EN_R = crate::BitReader<YC_EN_A>;
#[doc = "S-port Video enable Selection.\n\nThis bit selects whether the S-port(Y/C) video output is enabled or disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YC_EN_A {
    #[doc = "0: Y/C is disable"]
    Y_C = 0,
}
impl From<YC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: YC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl YC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<YC_EN_A> {
        match self.bits {
            false => Some(YC_EN_A::Y_C),
            _ => None,
        }
    }
    #[doc = "Y/C is disable"]
    #[inline(always)]
    pub fn is_y_c(&self) -> bool {
        *self == YC_EN_A::Y_C
    }
}
#[doc = "Field `yc_en` writer - S-port Video enable Selection.\n\nThis bit selects whether the S-port(Y/C) video output is enabled or disabled."]
pub type YC_EN_W<'a, REG> = crate::BitWriter<'a, REG, YC_EN_A>;
impl<'a, REG> YC_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Y/C is disable"]
    #[inline(always)]
    pub fn y_c(self) -> &'a mut crate::W<REG> {
        self.variant(YC_EN_A::Y_C)
    }
}
#[doc = "Field `yuv_rgb_output_en` reader - "]
pub type YUV_RGB_OUTPUT_EN_R = crate::BitReader<YUV_RGB_OUTPUT_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YUV_RGB_OUTPUT_EN_A {
    #[doc = "0: CVBS"]
    CVBS = 0,
}
impl From<YUV_RGB_OUTPUT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: YUV_RGB_OUTPUT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl YUV_RGB_OUTPUT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<YUV_RGB_OUTPUT_EN_A> {
        match self.bits {
            false => Some(YUV_RGB_OUTPUT_EN_A::CVBS),
            _ => None,
        }
    }
    #[doc = "CVBS"]
    #[inline(always)]
    pub fn is_cvbs(&self) -> bool {
        *self == YUV_RGB_OUTPUT_EN_A::CVBS
    }
}
#[doc = "Field `yuv_rgb_output_en` writer - "]
pub type YUV_RGB_OUTPUT_EN_W<'a, REG> = crate::BitWriter<'a, REG, YUV_RGB_OUTPUT_EN_A>;
impl<'a, REG> YUV_RGB_OUTPUT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CVBS"]
    #[inline(always)]
    pub fn cvbs(self) -> &'a mut crate::W<REG> {
        self.variant(YUV_RGB_OUTPUT_EN_A::CVBS)
    }
}
#[doc = "Field `input_chroma_data_sampling_rate_sel` reader - "]
pub type INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_R =
    crate::BitReader<INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_A {
    #[doc = "0: 4:4:4"]
    _4_4_4 = 0,
    #[doc = "1: 4:2:2"]
    _4_2_2 = 1,
}
impl From<INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_A {
        match self.bits {
            false => INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_A::_4_4_4,
            true => INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_A::_4_2_2,
        }
    }
    #[doc = "4:4:4"]
    #[inline(always)]
    pub fn is_4_4_4(&self) -> bool {
        *self == INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_A::_4_4_4
    }
    #[doc = "4:2:2"]
    #[inline(always)]
    pub fn is_4_2_2(&self) -> bool {
        *self == INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_A::_4_2_2
    }
}
#[doc = "Field `input_chroma_data_sampling_rate_sel` writer - "]
pub type INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_W<'a, REG> =
    crate::BitWriter<'a, REG, INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_A>;
impl<'a, REG> INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4:4:4"]
    #[inline(always)]
    pub fn _4_4_4(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_A::_4_4_4)
    }
    #[doc = "4:2:2"]
    #[inline(always)]
    pub fn _4_2_2(self) -> &'a mut crate::W<REG> {
        self.variant(INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_A::_4_2_2)
    }
}
#[doc = "Field `cb_cr_seq_for_422_mode` reader - Cb_Cr_Seq_For_422_Mode"]
pub type CB_CR_SEQ_FOR_422_MODE_R = crate::BitReader<CB_CR_SEQ_FOR_422_MODE_A>;
#[doc = "Cb_Cr_Seq_For_422_Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CB_CR_SEQ_FOR_422_MODE_A {
    #[doc = "0: Cb first"]
    C_B = 0,
    #[doc = "1: Cr first"]
    C_R = 1,
}
impl From<CB_CR_SEQ_FOR_422_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: CB_CR_SEQ_FOR_422_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl CB_CR_SEQ_FOR_422_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CB_CR_SEQ_FOR_422_MODE_A {
        match self.bits {
            false => CB_CR_SEQ_FOR_422_MODE_A::C_B,
            true => CB_CR_SEQ_FOR_422_MODE_A::C_R,
        }
    }
    #[doc = "Cb first"]
    #[inline(always)]
    pub fn is_c_b(&self) -> bool {
        *self == CB_CR_SEQ_FOR_422_MODE_A::C_B
    }
    #[doc = "Cr first"]
    #[inline(always)]
    pub fn is_c_r(&self) -> bool {
        *self == CB_CR_SEQ_FOR_422_MODE_A::C_R
    }
}
#[doc = "Field `cb_cr_seq_for_422_mode` writer - Cb_Cr_Seq_For_422_Mode"]
pub type CB_CR_SEQ_FOR_422_MODE_W<'a, REG> = crate::BitWriter<'a, REG, CB_CR_SEQ_FOR_422_MODE_A>;
impl<'a, REG> CB_CR_SEQ_FOR_422_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cb first"]
    #[inline(always)]
    pub fn c_b(self) -> &'a mut crate::W<REG> {
        self.variant(CB_CR_SEQ_FOR_422_MODE_A::C_B)
    }
    #[doc = "Cr first"]
    #[inline(always)]
    pub fn c_r(self) -> &'a mut crate::W<REG> {
        self.variant(CB_CR_SEQ_FOR_422_MODE_A::C_R)
    }
}
#[doc = "Field `core_control_logic_clock_sel` reader - "]
pub type CORE_CONTROL_LOGIC_CLOCK_SEL_R = crate::BitReader<CORE_CONTROL_LOGIC_CLOCK_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CORE_CONTROL_LOGIC_CLOCK_SEL_A {
    #[doc = "0: Using 27 MHz clock or 74.25 MHz clock depend on CCU setting"]
    _27M_74_25M = 0,
    #[doc = "1: Using 54 MHz clock or 148.5 MHz clock depend on CCU setting"]
    _54M_148_5M = 1,
}
impl From<CORE_CONTROL_LOGIC_CLOCK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CORE_CONTROL_LOGIC_CLOCK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CORE_CONTROL_LOGIC_CLOCK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CORE_CONTROL_LOGIC_CLOCK_SEL_A {
        match self.bits {
            false => CORE_CONTROL_LOGIC_CLOCK_SEL_A::_27M_74_25M,
            true => CORE_CONTROL_LOGIC_CLOCK_SEL_A::_54M_148_5M,
        }
    }
    #[doc = "Using 27 MHz clock or 74.25 MHz clock depend on CCU setting"]
    #[inline(always)]
    pub fn is_27m_74_25m(&self) -> bool {
        *self == CORE_CONTROL_LOGIC_CLOCK_SEL_A::_27M_74_25M
    }
    #[doc = "Using 54 MHz clock or 148.5 MHz clock depend on CCU setting"]
    #[inline(always)]
    pub fn is_54m_148_5m(&self) -> bool {
        *self == CORE_CONTROL_LOGIC_CLOCK_SEL_A::_54M_148_5M
    }
}
#[doc = "Field `core_control_logic_clock_sel` writer - "]
pub type CORE_CONTROL_LOGIC_CLOCK_SEL_W<'a, REG> =
    crate::BitWriter<'a, REG, CORE_CONTROL_LOGIC_CLOCK_SEL_A>;
impl<'a, REG> CORE_CONTROL_LOGIC_CLOCK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Using 27 MHz clock or 74.25 MHz clock depend on CCU setting"]
    #[inline(always)]
    pub fn _27m_74_25m(self) -> &'a mut crate::W<REG> {
        self.variant(CORE_CONTROL_LOGIC_CLOCK_SEL_A::_27M_74_25M)
    }
    #[doc = "Using 54 MHz clock or 148.5 MHz clock depend on CCU setting"]
    #[inline(always)]
    pub fn _54m_148_5m(self) -> &'a mut crate::W<REG> {
        self.variant(CORE_CONTROL_LOGIC_CLOCK_SEL_A::_54M_148_5M)
    }
}
#[doc = "Field `core_datapath_logic_clock_sel` reader - "]
pub type CORE_DATAPATH_LOGIC_CLOCK_SEL_R = crate::BitReader<CORE_DATAPATH_LOGIC_CLOCK_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CORE_DATAPATH_LOGIC_CLOCK_SEL_A {
    #[doc = "0: Using 27 MHz clock or 74.25 MHz clock depend on CCU setting"]
    _27M_74_25M = 0,
    #[doc = "1: Using 54 MHz clock or 148.5 MHz clock depend on CCU setting"]
    _54M_148_5M = 1,
}
impl From<CORE_DATAPATH_LOGIC_CLOCK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CORE_DATAPATH_LOGIC_CLOCK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CORE_DATAPATH_LOGIC_CLOCK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CORE_DATAPATH_LOGIC_CLOCK_SEL_A {
        match self.bits {
            false => CORE_DATAPATH_LOGIC_CLOCK_SEL_A::_27M_74_25M,
            true => CORE_DATAPATH_LOGIC_CLOCK_SEL_A::_54M_148_5M,
        }
    }
    #[doc = "Using 27 MHz clock or 74.25 MHz clock depend on CCU setting"]
    #[inline(always)]
    pub fn is_27m_74_25m(&self) -> bool {
        *self == CORE_DATAPATH_LOGIC_CLOCK_SEL_A::_27M_74_25M
    }
    #[doc = "Using 54 MHz clock or 148.5 MHz clock depend on CCU setting"]
    #[inline(always)]
    pub fn is_54m_148_5m(&self) -> bool {
        *self == CORE_DATAPATH_LOGIC_CLOCK_SEL_A::_54M_148_5M
    }
}
#[doc = "Field `core_datapath_logic_clock_sel` writer - "]
pub type CORE_DATAPATH_LOGIC_CLOCK_SEL_W<'a, REG> =
    crate::BitWriter<'a, REG, CORE_DATAPATH_LOGIC_CLOCK_SEL_A>;
impl<'a, REG> CORE_DATAPATH_LOGIC_CLOCK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Using 27 MHz clock or 74.25 MHz clock depend on CCU setting"]
    #[inline(always)]
    pub fn _27m_74_25m(self) -> &'a mut crate::W<REG> {
        self.variant(CORE_DATAPATH_LOGIC_CLOCK_SEL_A::_27M_74_25M)
    }
    #[doc = "Using 54 MHz clock or 148.5 MHz clock depend on CCU setting"]
    #[inline(always)]
    pub fn _54m_148_5m(self) -> &'a mut crate::W<REG> {
        self.variant(CORE_DATAPATH_LOGIC_CLOCK_SEL_A::_54M_148_5M)
    }
}
#[doc = "Field `dac_control_logic_clock_sel` reader - "]
pub type DAC_CONTROL_LOGIC_CLOCK_SEL_R = crate::BitReader<DAC_CONTROL_LOGIC_CLOCK_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_CONTROL_LOGIC_CLOCK_SEL_A {
    #[doc = "0: Using 27 MHz clock or 74.25 MHz clock depend on CCU setting"]
    _27M_74_25M = 0,
    #[doc = "1: Using 54 MHz clock or 148.5 MHz clock depend on CCU setting"]
    _54M_148_5M = 1,
}
impl From<DAC_CONTROL_LOGIC_CLOCK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DAC_CONTROL_LOGIC_CLOCK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC_CONTROL_LOGIC_CLOCK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC_CONTROL_LOGIC_CLOCK_SEL_A {
        match self.bits {
            false => DAC_CONTROL_LOGIC_CLOCK_SEL_A::_27M_74_25M,
            true => DAC_CONTROL_LOGIC_CLOCK_SEL_A::_54M_148_5M,
        }
    }
    #[doc = "Using 27 MHz clock or 74.25 MHz clock depend on CCU setting"]
    #[inline(always)]
    pub fn is_27m_74_25m(&self) -> bool {
        *self == DAC_CONTROL_LOGIC_CLOCK_SEL_A::_27M_74_25M
    }
    #[doc = "Using 54 MHz clock or 148.5 MHz clock depend on CCU setting"]
    #[inline(always)]
    pub fn is_54m_148_5m(&self) -> bool {
        *self == DAC_CONTROL_LOGIC_CLOCK_SEL_A::_54M_148_5M
    }
}
#[doc = "Field `dac_control_logic_clock_sel` writer - "]
pub type DAC_CONTROL_LOGIC_CLOCK_SEL_W<'a, REG> =
    crate::BitWriter<'a, REG, DAC_CONTROL_LOGIC_CLOCK_SEL_A>;
impl<'a, REG> DAC_CONTROL_LOGIC_CLOCK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Using 27 MHz clock or 74.25 MHz clock depend on CCU setting"]
    #[inline(always)]
    pub fn _27m_74_25m(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_CONTROL_LOGIC_CLOCK_SEL_A::_27M_74_25M)
    }
    #[doc = "Using 54 MHz clock or 148.5 MHz clock depend on CCU setting"]
    #[inline(always)]
    pub fn _54m_148_5m(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_CONTROL_LOGIC_CLOCK_SEL_A::_54M_148_5M)
    }
}
#[doc = "Field `dac_src_sel` reader - "]
pub type DAC_SRC_SEL_R = crate::FieldReader<DAC_SRC_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC_SRC_SEL_A {
    #[doc = "0: TV Encoder"]
    TVE = 0,
    #[doc = "1: LCD controller, override all other TV encoder setting, the DAC clock can from LCD controller."]
    LCD_CONTROLLER = 1,
    #[doc = "2: DAC test mode, DAC using DAC clock"]
    DAC_TEST = 2,
    #[doc = "3: DAC test mode, DAC using AHB clock"]
    DAC_TEST_AHB = 3,
}
impl From<DAC_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAC_SRC_SEL_A {
    type Ux = u8;
}
impl DAC_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC_SRC_SEL_A {
        match self.bits {
            0 => DAC_SRC_SEL_A::TVE,
            1 => DAC_SRC_SEL_A::LCD_CONTROLLER,
            2 => DAC_SRC_SEL_A::DAC_TEST,
            3 => DAC_SRC_SEL_A::DAC_TEST_AHB,
            _ => unreachable!(),
        }
    }
    #[doc = "TV Encoder"]
    #[inline(always)]
    pub fn is_tve(&self) -> bool {
        *self == DAC_SRC_SEL_A::TVE
    }
    #[doc = "LCD controller, override all other TV encoder setting, the DAC clock can from LCD controller."]
    #[inline(always)]
    pub fn is_lcd_controller(&self) -> bool {
        *self == DAC_SRC_SEL_A::LCD_CONTROLLER
    }
    #[doc = "DAC test mode, DAC using DAC clock"]
    #[inline(always)]
    pub fn is_dac_test(&self) -> bool {
        *self == DAC_SRC_SEL_A::DAC_TEST
    }
    #[doc = "DAC test mode, DAC using AHB clock"]
    #[inline(always)]
    pub fn is_dac_test_ahb(&self) -> bool {
        *self == DAC_SRC_SEL_A::DAC_TEST_AHB
    }
}
#[doc = "Field `dac_src_sel` writer - "]
pub type DAC_SRC_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DAC_SRC_SEL_A>;
impl<'a, REG> DAC_SRC_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TV Encoder"]
    #[inline(always)]
    pub fn tve(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_SRC_SEL_A::TVE)
    }
    #[doc = "LCD controller, override all other TV encoder setting, the DAC clock can from LCD controller."]
    #[inline(always)]
    pub fn lcd_controller(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_SRC_SEL_A::LCD_CONTROLLER)
    }
    #[doc = "DAC test mode, DAC using DAC clock"]
    #[inline(always)]
    pub fn dac_test(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_SRC_SEL_A::DAC_TEST)
    }
    #[doc = "DAC test mode, DAC using AHB clock"]
    #[inline(always)]
    pub fn dac_test_ahb(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_SRC_SEL_A::DAC_TEST_AHB)
    }
}
#[doc = "Field `bypass_tv` reader - "]
pub type BYPASS_TV_R = crate::BitReader<BYPASS_TV_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS_TV_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<BYPASS_TV_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_TV_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS_TV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BYPASS_TV_A {
        match self.bits {
            false => BYPASS_TV_A::DISABLE,
            true => BYPASS_TV_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BYPASS_TV_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BYPASS_TV_A::ENABLE
    }
}
#[doc = "Field `bypass_tv` writer - "]
pub type BYPASS_TV_W<'a, REG> = crate::BitWriter<'a, REG, BYPASS_TV_A>;
impl<'a, REG> BYPASS_TV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS_TV_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS_TV_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - TVMode_Select\n\nNote: Changing this register value will cause some relative register setting to relative value."]
    #[inline(always)]
    pub fn tvmode_select(&self) -> TVMODE_SELECT_R {
        TVMODE_SELECT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Mode_1080i_1250Line_Sel"]
    #[inline(always)]
    pub fn mode_1080i_1250line_sel(&self) -> MODE_1080I_1250LINE_SEL_R {
        MODE_1080I_1250LINE_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Standard Color bar input selection\n\nThis bit selects whether the Video Encoder video data input is replaced by an internal standard color bar generator or not."]
    #[inline(always)]
    pub fn color_bar_mode(&self) -> COLOR_BAR_MODE_R {
        COLOR_BAR_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn color_bar_type(&self) -> COLOR_BAR_TYPE_R {
        COLOR_BAR_TYPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Composite video enables selection\n\nThis bit selects whether the composite video output (CVBS) is enabled or disabled."]
    #[inline(always)]
    pub fn cvbs_en(&self) -> CVBS_EN_R {
        CVBS_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - S-port Video enable Selection.\n\nThis bit selects whether the S-port(Y/C) video output is enabled or disabled."]
    #[inline(always)]
    pub fn yc_en(&self) -> YC_EN_R {
        YC_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn yuv_rgb_output_en(&self) -> YUV_RGB_OUTPUT_EN_R {
        YUV_RGB_OUTPUT_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn input_chroma_data_sampling_rate_sel(&self) -> INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_R {
        INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Cb_Cr_Seq_For_422_Mode"]
    #[inline(always)]
    pub fn cb_cr_seq_for_422_mode(&self) -> CB_CR_SEQ_FOR_422_MODE_R {
        CB_CR_SEQ_FOR_422_MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn core_control_logic_clock_sel(&self) -> CORE_CONTROL_LOGIC_CLOCK_SEL_R {
        CORE_CONTROL_LOGIC_CLOCK_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn core_datapath_logic_clock_sel(&self) -> CORE_DATAPATH_LOGIC_CLOCK_SEL_R {
        CORE_DATAPATH_LOGIC_CLOCK_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dac_control_logic_clock_sel(&self) -> DAC_CONTROL_LOGIC_CLOCK_SEL_R {
        DAC_CONTROL_LOGIC_CLOCK_SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn dac_src_sel(&self) -> DAC_SRC_SEL_R {
        DAC_SRC_SEL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn bypass_tv(&self) -> BYPASS_TV_R {
        BYPASS_TV_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TVMode_Select\n\nNote: Changing this register value will cause some relative register setting to relative value."]
    #[inline(always)]
    #[must_use]
    pub fn tvmode_select(&mut self) -> TVMODE_SELECT_W<TVE_CONFIGURATION_SPEC> {
        TVMODE_SELECT_W::new(self, 0)
    }
    #[doc = "Bit 4 - Mode_1080i_1250Line_Sel"]
    #[inline(always)]
    #[must_use]
    pub fn mode_1080i_1250line_sel(&mut self) -> MODE_1080I_1250LINE_SEL_W<TVE_CONFIGURATION_SPEC> {
        MODE_1080I_1250LINE_SEL_W::new(self, 4)
    }
    #[doc = "Bit 8 - Standard Color bar input selection\n\nThis bit selects whether the Video Encoder video data input is replaced by an internal standard color bar generator or not."]
    #[inline(always)]
    #[must_use]
    pub fn color_bar_mode(&mut self) -> COLOR_BAR_MODE_W<TVE_CONFIGURATION_SPEC> {
        COLOR_BAR_MODE_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn color_bar_type(&mut self) -> COLOR_BAR_TYPE_W<TVE_CONFIGURATION_SPEC> {
        COLOR_BAR_TYPE_W::new(self, 9)
    }
    #[doc = "Bit 16 - Composite video enables selection\n\nThis bit selects whether the composite video output (CVBS) is enabled or disabled."]
    #[inline(always)]
    #[must_use]
    pub fn cvbs_en(&mut self) -> CVBS_EN_W<TVE_CONFIGURATION_SPEC> {
        CVBS_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - S-port Video enable Selection.\n\nThis bit selects whether the S-port(Y/C) video output is enabled or disabled."]
    #[inline(always)]
    #[must_use]
    pub fn yc_en(&mut self) -> YC_EN_W<TVE_CONFIGURATION_SPEC> {
        YC_EN_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn yuv_rgb_output_en(&mut self) -> YUV_RGB_OUTPUT_EN_W<TVE_CONFIGURATION_SPEC> {
        YUV_RGB_OUTPUT_EN_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn input_chroma_data_sampling_rate_sel(
        &mut self,
    ) -> INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_W<TVE_CONFIGURATION_SPEC> {
        INPUT_CHROMA_DATA_SAMPLING_RATE_SEL_W::new(self, 19)
    }
    #[doc = "Bit 20 - Cb_Cr_Seq_For_422_Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cb_cr_seq_for_422_mode(&mut self) -> CB_CR_SEQ_FOR_422_MODE_W<TVE_CONFIGURATION_SPEC> {
        CB_CR_SEQ_FOR_422_MODE_W::new(self, 20)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn core_control_logic_clock_sel(
        &mut self,
    ) -> CORE_CONTROL_LOGIC_CLOCK_SEL_W<TVE_CONFIGURATION_SPEC> {
        CORE_CONTROL_LOGIC_CLOCK_SEL_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn core_datapath_logic_clock_sel(
        &mut self,
    ) -> CORE_DATAPATH_LOGIC_CLOCK_SEL_W<TVE_CONFIGURATION_SPEC> {
        CORE_DATAPATH_LOGIC_CLOCK_SEL_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn dac_control_logic_clock_sel(
        &mut self,
    ) -> DAC_CONTROL_LOGIC_CLOCK_SEL_W<TVE_CONFIGURATION_SPEC> {
        DAC_CONTROL_LOGIC_CLOCK_SEL_W::new(self, 26)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    #[must_use]
    pub fn dac_src_sel(&mut self) -> DAC_SRC_SEL_W<TVE_CONFIGURATION_SPEC> {
        DAC_SRC_SEL_W::new(self, 27)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_tv(&mut self) -> BYPASS_TV_W<TVE_CONFIGURATION_SPEC> {
        BYPASS_TV_W::new(self, 29)
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
#[doc = "TV Encoder Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_configuration::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_configuration::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_CONFIGURATION_SPEC;
impl crate::RegisterSpec for TVE_CONFIGURATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_configuration::R`](R) reader structure"]
impl crate::Readable for TVE_CONFIGURATION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_configuration::W`](W) writer structure"]
impl crate::Writable for TVE_CONFIGURATION_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_configuration to value 0x0001_0000"]
impl crate::Resettable for TVE_CONFIGURATION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
