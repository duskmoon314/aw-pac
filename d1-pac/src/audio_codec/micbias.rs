#[doc = "Register `micbias` reader"]
pub type R = crate::R<MICBIAS_SPEC>;
#[doc = "Register `micbias` writer"]
pub type W = crate::W<MICBIAS_SPEC>;
#[doc = "Field `mmic_bias_chopper_clk_sel` reader - MMIC BIAS Chopper Clock Select"]
pub type MMIC_BIAS_CHOPPER_CLK_SEL_R = crate::FieldReader<MMIC_BIAS_CHOPPER_CLK_SEL_A>;
#[doc = "MMIC BIAS Chopper Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMIC_BIAS_CHOPPER_CLK_SEL_A {
    #[doc = "0: 250 kHz"]
    _250KHZ = 0,
    #[doc = "1: 500 kHz"]
    _500KHZ = 1,
    #[doc = "2: 1 MHz"]
    _1MHZ = 2,
    #[doc = "3: 2 MHz"]
    _2MHZ = 3,
}
impl From<MMIC_BIAS_CHOPPER_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MMIC_BIAS_CHOPPER_CLK_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MMIC_BIAS_CHOPPER_CLK_SEL_A {
    type Ux = u8;
}
impl MMIC_BIAS_CHOPPER_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMIC_BIAS_CHOPPER_CLK_SEL_A {
        match self.bits {
            0 => MMIC_BIAS_CHOPPER_CLK_SEL_A::_250KHZ,
            1 => MMIC_BIAS_CHOPPER_CLK_SEL_A::_500KHZ,
            2 => MMIC_BIAS_CHOPPER_CLK_SEL_A::_1MHZ,
            3 => MMIC_BIAS_CHOPPER_CLK_SEL_A::_2MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "250 kHz"]
    #[inline(always)]
    pub fn is_250khz(&self) -> bool {
        *self == MMIC_BIAS_CHOPPER_CLK_SEL_A::_250KHZ
    }
    #[doc = "500 kHz"]
    #[inline(always)]
    pub fn is_500khz(&self) -> bool {
        *self == MMIC_BIAS_CHOPPER_CLK_SEL_A::_500KHZ
    }
    #[doc = "1 MHz"]
    #[inline(always)]
    pub fn is_1mhz(&self) -> bool {
        *self == MMIC_BIAS_CHOPPER_CLK_SEL_A::_1MHZ
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn is_2mhz(&self) -> bool {
        *self == MMIC_BIAS_CHOPPER_CLK_SEL_A::_2MHZ
    }
}
#[doc = "Field `mmic_bias_chopper_clk_sel` writer - MMIC BIAS Chopper Clock Select"]
pub type MMIC_BIAS_CHOPPER_CLK_SEL_W<'a, REG> =
    crate::FieldWriterSafe<'a, REG, 2, MMIC_BIAS_CHOPPER_CLK_SEL_A>;
impl<'a, REG> MMIC_BIAS_CHOPPER_CLK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "250 kHz"]
    #[inline(always)]
    pub fn _250khz(self) -> &'a mut crate::W<REG> {
        self.variant(MMIC_BIAS_CHOPPER_CLK_SEL_A::_250KHZ)
    }
    #[doc = "500 kHz"]
    #[inline(always)]
    pub fn _500khz(self) -> &'a mut crate::W<REG> {
        self.variant(MMIC_BIAS_CHOPPER_CLK_SEL_A::_500KHZ)
    }
    #[doc = "1 MHz"]
    #[inline(always)]
    pub fn _1mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MMIC_BIAS_CHOPPER_CLK_SEL_A::_1MHZ)
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn _2mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MMIC_BIAS_CHOPPER_CLK_SEL_A::_2MHZ)
    }
}
#[doc = "Field `mmic_bias_chopper_en` reader - MMIC BIAS Chopper Enable"]
pub type MMIC_BIAS_CHOPPER_EN_R = crate::BitReader<MMIC_BIAS_CHOPPER_EN_A>;
#[doc = "MMIC BIAS Chopper Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMIC_BIAS_CHOPPER_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<MMIC_BIAS_CHOPPER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MMIC_BIAS_CHOPPER_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl MMIC_BIAS_CHOPPER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMIC_BIAS_CHOPPER_EN_A {
        match self.bits {
            false => MMIC_BIAS_CHOPPER_EN_A::DISABLED,
            true => MMIC_BIAS_CHOPPER_EN_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MMIC_BIAS_CHOPPER_EN_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MMIC_BIAS_CHOPPER_EN_A::ENABLED
    }
}
#[doc = "Field `mmic_bias_chopper_en` writer - MMIC BIAS Chopper Enable"]
pub type MMIC_BIAS_CHOPPER_EN_W<'a, REG> = crate::BitWriter<'a, REG, MMIC_BIAS_CHOPPER_EN_A>;
impl<'a, REG> MMIC_BIAS_CHOPPER_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MMIC_BIAS_CHOPPER_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MMIC_BIAS_CHOPPER_EN_A::ENABLED)
    }
}
#[doc = "Field `mbiassel` reader - MMICBIAS Voltage Level Select"]
pub type MBIASSEL_R = crate::FieldReader<MBIASSEL_A>;
#[doc = "MMICBIAS Voltage Level Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MBIASSEL_A {
    #[doc = "0: 1.88 V"]
    _1_88_V = 0,
    #[doc = "1: 2.09 V"]
    _2_09_V = 1,
    #[doc = "2: 2.33 V"]
    _2_33_V = 2,
    #[doc = "3: 2.50 V"]
    _2_50_V = 3,
}
impl From<MBIASSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MBIASSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MBIASSEL_A {
    type Ux = u8;
}
impl MBIASSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MBIASSEL_A {
        match self.bits {
            0 => MBIASSEL_A::_1_88_V,
            1 => MBIASSEL_A::_2_09_V,
            2 => MBIASSEL_A::_2_33_V,
            3 => MBIASSEL_A::_2_50_V,
            _ => unreachable!(),
        }
    }
    #[doc = "1.88 V"]
    #[inline(always)]
    pub fn is_1_88_v(&self) -> bool {
        *self == MBIASSEL_A::_1_88_V
    }
    #[doc = "2.09 V"]
    #[inline(always)]
    pub fn is_2_09_v(&self) -> bool {
        *self == MBIASSEL_A::_2_09_V
    }
    #[doc = "2.33 V"]
    #[inline(always)]
    pub fn is_2_33_v(&self) -> bool {
        *self == MBIASSEL_A::_2_33_V
    }
    #[doc = "2.50 V"]
    #[inline(always)]
    pub fn is_2_50_v(&self) -> bool {
        *self == MBIASSEL_A::_2_50_V
    }
}
#[doc = "Field `mbiassel` writer - MMICBIAS Voltage Level Select"]
pub type MBIASSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MBIASSEL_A>;
impl<'a, REG> MBIASSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.88 V"]
    #[inline(always)]
    pub fn _1_88_v(self) -> &'a mut crate::W<REG> {
        self.variant(MBIASSEL_A::_1_88_V)
    }
    #[doc = "2.09 V"]
    #[inline(always)]
    pub fn _2_09_v(self) -> &'a mut crate::W<REG> {
        self.variant(MBIASSEL_A::_2_09_V)
    }
    #[doc = "2.33 V"]
    #[inline(always)]
    pub fn _2_33_v(self) -> &'a mut crate::W<REG> {
        self.variant(MBIASSEL_A::_2_33_V)
    }
    #[doc = "2.50 V"]
    #[inline(always)]
    pub fn _2_50_v(self) -> &'a mut crate::W<REG> {
        self.variant(MBIASSEL_A::_2_50_V)
    }
}
#[doc = "Field `mmicbiasen` reader - Master Microphone Bias Enable"]
pub type MMICBIASEN_R = crate::BitReader<MMICBIASEN_A>;
#[doc = "Master Microphone Bias Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMICBIASEN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<MMICBIASEN_A> for bool {
    #[inline(always)]
    fn from(variant: MMICBIASEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MMICBIASEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMICBIASEN_A {
        match self.bits {
            false => MMICBIASEN_A::DISABLED,
            true => MMICBIASEN_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MMICBIASEN_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MMICBIASEN_A::ENABLED
    }
}
#[doc = "Field `mmicbiasen` writer - Master Microphone Bias Enable"]
pub type MMICBIASEN_W<'a, REG> = crate::BitWriter<'a, REG, MMICBIASEN_A>;
impl<'a, REG> MMICBIASEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MMICBIASEN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MMICBIASEN_A::ENABLED)
    }
}
#[doc = "Field `hmic_bias_chopper_clk_sel` reader - HMIC BIAS Chopper Clock Select"]
pub type HMIC_BIAS_CHOPPER_CLK_SEL_R = crate::FieldReader<HMIC_BIAS_CHOPPER_CLK_SEL_A>;
#[doc = "HMIC BIAS Chopper Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HMIC_BIAS_CHOPPER_CLK_SEL_A {
    #[doc = "0: 250 kHz"]
    _250KHZ = 0,
    #[doc = "1: 500 kHz"]
    _500KHZ = 1,
    #[doc = "2: 1 MHz"]
    _1MHZ = 2,
    #[doc = "3: 2 MHz"]
    _2MHZ = 3,
}
impl From<HMIC_BIAS_CHOPPER_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HMIC_BIAS_CHOPPER_CLK_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HMIC_BIAS_CHOPPER_CLK_SEL_A {
    type Ux = u8;
}
impl HMIC_BIAS_CHOPPER_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HMIC_BIAS_CHOPPER_CLK_SEL_A {
        match self.bits {
            0 => HMIC_BIAS_CHOPPER_CLK_SEL_A::_250KHZ,
            1 => HMIC_BIAS_CHOPPER_CLK_SEL_A::_500KHZ,
            2 => HMIC_BIAS_CHOPPER_CLK_SEL_A::_1MHZ,
            3 => HMIC_BIAS_CHOPPER_CLK_SEL_A::_2MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "250 kHz"]
    #[inline(always)]
    pub fn is_250khz(&self) -> bool {
        *self == HMIC_BIAS_CHOPPER_CLK_SEL_A::_250KHZ
    }
    #[doc = "500 kHz"]
    #[inline(always)]
    pub fn is_500khz(&self) -> bool {
        *self == HMIC_BIAS_CHOPPER_CLK_SEL_A::_500KHZ
    }
    #[doc = "1 MHz"]
    #[inline(always)]
    pub fn is_1mhz(&self) -> bool {
        *self == HMIC_BIAS_CHOPPER_CLK_SEL_A::_1MHZ
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn is_2mhz(&self) -> bool {
        *self == HMIC_BIAS_CHOPPER_CLK_SEL_A::_2MHZ
    }
}
#[doc = "Field `hmic_bias_chopper_clk_sel` writer - HMIC BIAS Chopper Clock Select"]
pub type HMIC_BIAS_CHOPPER_CLK_SEL_W<'a, REG> =
    crate::FieldWriterSafe<'a, REG, 2, HMIC_BIAS_CHOPPER_CLK_SEL_A>;
impl<'a, REG> HMIC_BIAS_CHOPPER_CLK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "250 kHz"]
    #[inline(always)]
    pub fn _250khz(self) -> &'a mut crate::W<REG> {
        self.variant(HMIC_BIAS_CHOPPER_CLK_SEL_A::_250KHZ)
    }
    #[doc = "500 kHz"]
    #[inline(always)]
    pub fn _500khz(self) -> &'a mut crate::W<REG> {
        self.variant(HMIC_BIAS_CHOPPER_CLK_SEL_A::_500KHZ)
    }
    #[doc = "1 MHz"]
    #[inline(always)]
    pub fn _1mhz(self) -> &'a mut crate::W<REG> {
        self.variant(HMIC_BIAS_CHOPPER_CLK_SEL_A::_1MHZ)
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn _2mhz(self) -> &'a mut crate::W<REG> {
        self.variant(HMIC_BIAS_CHOPPER_CLK_SEL_A::_2MHZ)
    }
}
#[doc = "Field `hmic_bias_chopper_en` reader - HMIC BIAS Chopper Enable"]
pub type HMIC_BIAS_CHOPPER_EN_R = crate::BitReader<HMIC_BIAS_CHOPPER_EN_A>;
#[doc = "HMIC BIAS Chopper Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HMIC_BIAS_CHOPPER_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<HMIC_BIAS_CHOPPER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HMIC_BIAS_CHOPPER_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HMIC_BIAS_CHOPPER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HMIC_BIAS_CHOPPER_EN_A {
        match self.bits {
            false => HMIC_BIAS_CHOPPER_EN_A::DISABLED,
            true => HMIC_BIAS_CHOPPER_EN_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HMIC_BIAS_CHOPPER_EN_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HMIC_BIAS_CHOPPER_EN_A::ENABLED
    }
}
#[doc = "Field `hmic_bias_chopper_en` writer - HMIC BIAS Chopper Enable"]
pub type HMIC_BIAS_CHOPPER_EN_W<'a, REG> = crate::BitWriter<'a, REG, HMIC_BIAS_CHOPPER_EN_A>;
impl<'a, REG> HMIC_BIAS_CHOPPER_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HMIC_BIAS_CHOPPER_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HMIC_BIAS_CHOPPER_EN_A::ENABLED)
    }
}
#[doc = "Field `hbiassel` reader - HMICBIAS Voltage Level Select"]
pub type HBIASSEL_R = crate::FieldReader<HBIASSEL_A>;
#[doc = "HMICBIAS Voltage Level Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HBIASSEL_A {
    #[doc = "0: 1.88 V"]
    _1_88_V = 0,
    #[doc = "1: 2.09 V"]
    _2_09_V = 1,
    #[doc = "2: 2.33 V"]
    _2_33_V = 2,
    #[doc = "3: 2.55 V"]
    _2_55_V = 3,
}
impl From<HBIASSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HBIASSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HBIASSEL_A {
    type Ux = u8;
}
impl HBIASSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HBIASSEL_A {
        match self.bits {
            0 => HBIASSEL_A::_1_88_V,
            1 => HBIASSEL_A::_2_09_V,
            2 => HBIASSEL_A::_2_33_V,
            3 => HBIASSEL_A::_2_55_V,
            _ => unreachable!(),
        }
    }
    #[doc = "1.88 V"]
    #[inline(always)]
    pub fn is_1_88_v(&self) -> bool {
        *self == HBIASSEL_A::_1_88_V
    }
    #[doc = "2.09 V"]
    #[inline(always)]
    pub fn is_2_09_v(&self) -> bool {
        *self == HBIASSEL_A::_2_09_V
    }
    #[doc = "2.33 V"]
    #[inline(always)]
    pub fn is_2_33_v(&self) -> bool {
        *self == HBIASSEL_A::_2_33_V
    }
    #[doc = "2.55 V"]
    #[inline(always)]
    pub fn is_2_55_v(&self) -> bool {
        *self == HBIASSEL_A::_2_55_V
    }
}
#[doc = "Field `hbiassel` writer - HMICBIAS Voltage Level Select"]
pub type HBIASSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, HBIASSEL_A>;
impl<'a, REG> HBIASSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.88 V"]
    #[inline(always)]
    pub fn _1_88_v(self) -> &'a mut crate::W<REG> {
        self.variant(HBIASSEL_A::_1_88_V)
    }
    #[doc = "2.09 V"]
    #[inline(always)]
    pub fn _2_09_v(self) -> &'a mut crate::W<REG> {
        self.variant(HBIASSEL_A::_2_09_V)
    }
    #[doc = "2.33 V"]
    #[inline(always)]
    pub fn _2_33_v(self) -> &'a mut crate::W<REG> {
        self.variant(HBIASSEL_A::_2_33_V)
    }
    #[doc = "2.55 V"]
    #[inline(always)]
    pub fn _2_55_v(self) -> &'a mut crate::W<REG> {
        self.variant(HBIASSEL_A::_2_55_V)
    }
}
#[doc = "Field `hmicbiasen` reader - Headphone Microphone Bias Enable"]
pub type HMICBIASEN_R = crate::BitReader<HMICBIASEN_A>;
#[doc = "Headphone Microphone Bias Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HMICBIASEN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<HMICBIASEN_A> for bool {
    #[inline(always)]
    fn from(variant: HMICBIASEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HMICBIASEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HMICBIASEN_A {
        match self.bits {
            false => HMICBIASEN_A::DISABLED,
            true => HMICBIASEN_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HMICBIASEN_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HMICBIASEN_A::ENABLED
    }
}
#[doc = "Field `hmicbiasen` writer - Headphone Microphone Bias Enable"]
pub type HMICBIASEN_W<'a, REG> = crate::BitWriter<'a, REG, HMICBIASEN_A>;
impl<'a, REG> HMICBIASEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HMICBIASEN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HMICBIASEN_A::ENABLED)
    }
}
#[doc = "Field `micdetpl` reader - When this bit is 1and AUTOPLEN is 0, the MICDET is pulled down to GND."]
pub type MICDETPL_R = crate::BitReader;
#[doc = "Field `micdetpl` writer - When this bit is 1and AUTOPLEN is 0, the MICDET is pulled down to GND."]
pub type MICDETPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `autoplen` reader - Enable the function to auto pull low MICDET when jack removal"]
pub type AUTOPLEN_R = crate::BitReader<AUTOPLEN_A>;
#[doc = "Enable the function to auto pull low MICDET when jack removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOPLEN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<AUTOPLEN_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOPLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOPLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AUTOPLEN_A {
        match self.bits {
            false => AUTOPLEN_A::DISABLED,
            true => AUTOPLEN_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOPLEN_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOPLEN_A::ENABLED
    }
}
#[doc = "Field `autoplen` writer - Enable the function to auto pull low MICDET when jack removal"]
pub type AUTOPLEN_W<'a, REG> = crate::BitWriter<'a, REG, AUTOPLEN_A>;
impl<'a, REG> AUTOPLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOPLEN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOPLEN_A::ENABLED)
    }
}
#[doc = "Field `det_mode` reader - MIC Detect Mode"]
pub type DET_MODE_R = crate::BitReader<DET_MODE_A>;
#[doc = "MIC Detect Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DET_MODE_A {
    #[doc = "0: Jack in pull low"]
    LOW = 0,
    #[doc = "1: Jack in pull high"]
    HIGH = 1,
}
impl From<DET_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: DET_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DET_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DET_MODE_A {
        match self.bits {
            false => DET_MODE_A::LOW,
            true => DET_MODE_A::HIGH,
        }
    }
    #[doc = "Jack in pull low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DET_MODE_A::LOW
    }
    #[doc = "Jack in pull high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DET_MODE_A::HIGH
    }
}
#[doc = "Field `det_mode` writer - MIC Detect Mode"]
pub type DET_MODE_W<'a, REG> = crate::BitWriter<'a, REG, DET_MODE_A>;
impl<'a, REG> DET_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Jack in pull low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(DET_MODE_A::LOW)
    }
    #[doc = "Jack in pull high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(DET_MODE_A::HIGH)
    }
}
#[doc = "Field `popfree` reader - When this bit is 0, HBIAS MICADC is controlled by registor"]
pub type POPFREE_R = crate::BitReader;
#[doc = "Field `popfree` writer - When this bit is 0, HBIAS MICADC is controlled by registor"]
pub type POPFREE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `micadcen` reader - Microphone detect ADC enable"]
pub type MICADCEN_R = crate::BitReader<MICADCEN_A>;
#[doc = "Microphone detect ADC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MICADCEN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<MICADCEN_A> for bool {
    #[inline(always)]
    fn from(variant: MICADCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MICADCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MICADCEN_A {
        match self.bits {
            false => MICADCEN_A::DISABLED,
            true => MICADCEN_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MICADCEN_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MICADCEN_A::ENABLED
    }
}
#[doc = "Field `micadcen` writer - Microphone detect ADC enable"]
pub type MICADCEN_W<'a, REG> = crate::BitWriter<'a, REG, MICADCEN_A>;
impl<'a, REG> MICADCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MICADCEN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MICADCEN_A::ENABLED)
    }
}
#[doc = "Field `seldetadcdy` reader - Select the delay time to pull low the micdet when jack removal"]
pub type SELDETADCDY_R = crate::FieldReader<SELDETADCDY_A>;
#[doc = "Select the delay time to pull low the micdet when jack removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELDETADCDY_A {
    #[doc = "0: 0.5 ms"]
    _0_5_MS = 0,
    #[doc = "1: 1 ms"]
    _1_MS = 1,
    #[doc = "2: 1.5 ms"]
    _1_5_MS = 2,
    #[doc = "3: 2 ms"]
    _2_MS = 3,
}
impl From<SELDETADCDY_A> for u8 {
    #[inline(always)]
    fn from(variant: SELDETADCDY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SELDETADCDY_A {
    type Ux = u8;
}
impl SELDETADCDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELDETADCDY_A {
        match self.bits {
            0 => SELDETADCDY_A::_0_5_MS,
            1 => SELDETADCDY_A::_1_MS,
            2 => SELDETADCDY_A::_1_5_MS,
            3 => SELDETADCDY_A::_2_MS,
            _ => unreachable!(),
        }
    }
    #[doc = "0.5 ms"]
    #[inline(always)]
    pub fn is_0_5_ms(&self) -> bool {
        *self == SELDETADCDY_A::_0_5_MS
    }
    #[doc = "1 ms"]
    #[inline(always)]
    pub fn is_1_ms(&self) -> bool {
        *self == SELDETADCDY_A::_1_MS
    }
    #[doc = "1.5 ms"]
    #[inline(always)]
    pub fn is_1_5_ms(&self) -> bool {
        *self == SELDETADCDY_A::_1_5_MS
    }
    #[doc = "2 ms"]
    #[inline(always)]
    pub fn is_2_ms(&self) -> bool {
        *self == SELDETADCDY_A::_2_MS
    }
}
#[doc = "Field `seldetadcdy` writer - Select the delay time to pull low the micdet when jack removal"]
pub type SELDETADCDY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SELDETADCDY_A>;
impl<'a, REG> SELDETADCDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.5 ms"]
    #[inline(always)]
    pub fn _0_5_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SELDETADCDY_A::_0_5_MS)
    }
    #[doc = "1 ms"]
    #[inline(always)]
    pub fn _1_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SELDETADCDY_A::_1_MS)
    }
    #[doc = "1.5 ms"]
    #[inline(always)]
    pub fn _1_5_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SELDETADCDY_A::_1_5_MS)
    }
    #[doc = "2 ms"]
    #[inline(always)]
    pub fn _2_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SELDETADCDY_A::_2_MS)
    }
}
#[doc = "Field `jackdeten` reader - Jack detect enable"]
pub type JACKDETEN_R = crate::BitReader<JACKDETEN_A>;
#[doc = "Jack detect enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JACKDETEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<JACKDETEN_A> for bool {
    #[inline(always)]
    fn from(variant: JACKDETEN_A) -> Self {
        variant as u8 != 0
    }
}
impl JACKDETEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JACKDETEN_A {
        match self.bits {
            false => JACKDETEN_A::DISABLE,
            true => JACKDETEN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == JACKDETEN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == JACKDETEN_A::ENABLE
    }
}
#[doc = "Field `jackdeten` writer - Jack detect enable"]
pub type JACKDETEN_W<'a, REG> = crate::BitWriter<'a, REG, JACKDETEN_A>;
impl<'a, REG> JACKDETEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(JACKDETEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(JACKDETEN_A::ENABLE)
    }
}
#[doc = "Field `seldetadcbf` reader - Select the time to enable HBIAS before MICADC work"]
pub type SELDETADCBF_R = crate::FieldReader<SELDETADCBF_A>;
#[doc = "Select the time to enable HBIAS before MICADC work\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELDETADCBF_A {
    #[doc = "0: 2 ms"]
    _2_MS = 0,
    #[doc = "1: 4 ms"]
    _4_MS = 1,
    #[doc = "2: 8 ms"]
    _8_MS = 2,
    #[doc = "3: 16 ms"]
    _16_MS = 3,
}
impl From<SELDETADCBF_A> for u8 {
    #[inline(always)]
    fn from(variant: SELDETADCBF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SELDETADCBF_A {
    type Ux = u8;
}
impl SELDETADCBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELDETADCBF_A {
        match self.bits {
            0 => SELDETADCBF_A::_2_MS,
            1 => SELDETADCBF_A::_4_MS,
            2 => SELDETADCBF_A::_8_MS,
            3 => SELDETADCBF_A::_16_MS,
            _ => unreachable!(),
        }
    }
    #[doc = "2 ms"]
    #[inline(always)]
    pub fn is_2_ms(&self) -> bool {
        *self == SELDETADCBF_A::_2_MS
    }
    #[doc = "4 ms"]
    #[inline(always)]
    pub fn is_4_ms(&self) -> bool {
        *self == SELDETADCBF_A::_4_MS
    }
    #[doc = "8 ms"]
    #[inline(always)]
    pub fn is_8_ms(&self) -> bool {
        *self == SELDETADCBF_A::_8_MS
    }
    #[doc = "16 ms"]
    #[inline(always)]
    pub fn is_16_ms(&self) -> bool {
        *self == SELDETADCBF_A::_16_MS
    }
}
#[doc = "Field `seldetadcbf` writer - Select the time to enable HBIAS before MICADC work"]
pub type SELDETADCBF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SELDETADCBF_A>;
impl<'a, REG> SELDETADCBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2 ms"]
    #[inline(always)]
    pub fn _2_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SELDETADCBF_A::_2_MS)
    }
    #[doc = "4 ms"]
    #[inline(always)]
    pub fn _4_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SELDETADCBF_A::_4_MS)
    }
    #[doc = "8 ms"]
    #[inline(always)]
    pub fn _8_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SELDETADCBF_A::_8_MS)
    }
    #[doc = "16 ms"]
    #[inline(always)]
    pub fn _16_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SELDETADCBF_A::_16_MS)
    }
}
#[doc = "Field `seldetadcdb` reader - Select debounce time when jack removal"]
pub type SELDETADCDB_R = crate::FieldReader<SELDETADCDB_A>;
#[doc = "Select debounce time when jack removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELDETADCDB_A {
    #[doc = "0: 128 ms"]
    _128_MS = 0,
    #[doc = "1: 256 ms"]
    _256_MS = 1,
    #[doc = "2: 512 ms"]
    _512_MS = 2,
    #[doc = "3: 1024 ms"]
    _1024_MS = 3,
}
impl From<SELDETADCDB_A> for u8 {
    #[inline(always)]
    fn from(variant: SELDETADCDB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SELDETADCDB_A {
    type Ux = u8;
}
impl SELDETADCDB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELDETADCDB_A {
        match self.bits {
            0 => SELDETADCDB_A::_128_MS,
            1 => SELDETADCDB_A::_256_MS,
            2 => SELDETADCDB_A::_512_MS,
            3 => SELDETADCDB_A::_1024_MS,
            _ => unreachable!(),
        }
    }
    #[doc = "128 ms"]
    #[inline(always)]
    pub fn is_128_ms(&self) -> bool {
        *self == SELDETADCDB_A::_128_MS
    }
    #[doc = "256 ms"]
    #[inline(always)]
    pub fn is_256_ms(&self) -> bool {
        *self == SELDETADCDB_A::_256_MS
    }
    #[doc = "512 ms"]
    #[inline(always)]
    pub fn is_512_ms(&self) -> bool {
        *self == SELDETADCDB_A::_512_MS
    }
    #[doc = "1024 ms"]
    #[inline(always)]
    pub fn is_1024_ms(&self) -> bool {
        *self == SELDETADCDB_A::_1024_MS
    }
}
#[doc = "Field `seldetadcdb` writer - Select debounce time when jack removal"]
pub type SELDETADCDB_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SELDETADCDB_A>;
impl<'a, REG> SELDETADCDB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128 ms"]
    #[inline(always)]
    pub fn _128_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SELDETADCDB_A::_128_MS)
    }
    #[doc = "256 ms"]
    #[inline(always)]
    pub fn _256_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SELDETADCDB_A::_256_MS)
    }
    #[doc = "512 ms"]
    #[inline(always)]
    pub fn _512_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SELDETADCDB_A::_512_MS)
    }
    #[doc = "1024 ms"]
    #[inline(always)]
    pub fn _1024_ms(self) -> &'a mut crate::W<REG> {
        self.variant(SELDETADCDB_A::_1024_MS)
    }
}
#[doc = "Field `seldetadcfs` reader - Select sample interval of the ADC sample\\\\ 2 ^ (SELDETADCFS + 1) ms"]
pub type SELDETADCFS_R = crate::FieldReader;
#[doc = "Field `seldetadcfs` writer - Select sample interval of the ADC sample\\\\ 2 ^ (SELDETADCFS + 1) ms"]
pub type SELDETADCFS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 2:3 - MMIC BIAS Chopper Clock Select"]
    #[inline(always)]
    pub fn mmic_bias_chopper_clk_sel(&self) -> MMIC_BIAS_CHOPPER_CLK_SEL_R {
        MMIC_BIAS_CHOPPER_CLK_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - MMIC BIAS Chopper Enable"]
    #[inline(always)]
    pub fn mmic_bias_chopper_en(&self) -> MMIC_BIAS_CHOPPER_EN_R {
        MMIC_BIAS_CHOPPER_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - MMICBIAS Voltage Level Select"]
    #[inline(always)]
    pub fn mbiassel(&self) -> MBIASSEL_R {
        MBIASSEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Master Microphone Bias Enable"]
    #[inline(always)]
    pub fn mmicbiasen(&self) -> MMICBIASEN_R {
        MMICBIASEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 10:11 - HMIC BIAS Chopper Clock Select"]
    #[inline(always)]
    pub fn hmic_bias_chopper_clk_sel(&self) -> HMIC_BIAS_CHOPPER_CLK_SEL_R {
        HMIC_BIAS_CHOPPER_CLK_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - HMIC BIAS Chopper Enable"]
    #[inline(always)]
    pub fn hmic_bias_chopper_en(&self) -> HMIC_BIAS_CHOPPER_EN_R {
        HMIC_BIAS_CHOPPER_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - HMICBIAS Voltage Level Select"]
    #[inline(always)]
    pub fn hbiassel(&self) -> HBIASSEL_R {
        HBIASSEL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Headphone Microphone Bias Enable"]
    #[inline(always)]
    pub fn hmicbiasen(&self) -> HMICBIASEN_R {
        HMICBIASEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When this bit is 1and AUTOPLEN is 0, the MICDET is pulled down to GND."]
    #[inline(always)]
    pub fn micdetpl(&self) -> MICDETPL_R {
        MICDETPL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable the function to auto pull low MICDET when jack removal"]
    #[inline(always)]
    pub fn autoplen(&self) -> AUTOPLEN_R {
        AUTOPLEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MIC Detect Mode"]
    #[inline(always)]
    pub fn det_mode(&self) -> DET_MODE_R {
        DET_MODE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When this bit is 0, HBIAS MICADC is controlled by registor"]
    #[inline(always)]
    pub fn popfree(&self) -> POPFREE_R {
        POPFREE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Microphone detect ADC enable"]
    #[inline(always)]
    pub fn micadcen(&self) -> MICADCEN_R {
        MICADCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Select the delay time to pull low the micdet when jack removal"]
    #[inline(always)]
    pub fn seldetadcdy(&self) -> SELDETADCDY_R {
        SELDETADCDY_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Jack detect enable"]
    #[inline(always)]
    pub fn jackdeten(&self) -> JACKDETEN_R {
        JACKDETEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Select the time to enable HBIAS before MICADC work"]
    #[inline(always)]
    pub fn seldetadcbf(&self) -> SELDETADCBF_R {
        SELDETADCBF_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Select debounce time when jack removal"]
    #[inline(always)]
    pub fn seldetadcdb(&self) -> SELDETADCDB_R {
        SELDETADCDB_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:30 - Select sample interval of the ADC sample\\\\ 2 ^ (SELDETADCFS + 1) ms"]
    #[inline(always)]
    pub fn seldetadcfs(&self) -> SELDETADCFS_R {
        SELDETADCFS_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - MMIC BIAS Chopper Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn mmic_bias_chopper_clk_sel(&mut self) -> MMIC_BIAS_CHOPPER_CLK_SEL_W<MICBIAS_SPEC> {
        MMIC_BIAS_CHOPPER_CLK_SEL_W::new(self, 2)
    }
    #[doc = "Bit 4 - MMIC BIAS Chopper Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mmic_bias_chopper_en(&mut self) -> MMIC_BIAS_CHOPPER_EN_W<MICBIAS_SPEC> {
        MMIC_BIAS_CHOPPER_EN_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - MMICBIAS Voltage Level Select"]
    #[inline(always)]
    #[must_use]
    pub fn mbiassel(&mut self) -> MBIASSEL_W<MICBIAS_SPEC> {
        MBIASSEL_W::new(self, 5)
    }
    #[doc = "Bit 7 - Master Microphone Bias Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mmicbiasen(&mut self) -> MMICBIASEN_W<MICBIAS_SPEC> {
        MMICBIASEN_W::new(self, 7)
    }
    #[doc = "Bits 10:11 - HMIC BIAS Chopper Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn hmic_bias_chopper_clk_sel(&mut self) -> HMIC_BIAS_CHOPPER_CLK_SEL_W<MICBIAS_SPEC> {
        HMIC_BIAS_CHOPPER_CLK_SEL_W::new(self, 10)
    }
    #[doc = "Bit 12 - HMIC BIAS Chopper Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hmic_bias_chopper_en(&mut self) -> HMIC_BIAS_CHOPPER_EN_W<MICBIAS_SPEC> {
        HMIC_BIAS_CHOPPER_EN_W::new(self, 12)
    }
    #[doc = "Bits 13:14 - HMICBIAS Voltage Level Select"]
    #[inline(always)]
    #[must_use]
    pub fn hbiassel(&mut self) -> HBIASSEL_W<MICBIAS_SPEC> {
        HBIASSEL_W::new(self, 13)
    }
    #[doc = "Bit 15 - Headphone Microphone Bias Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hmicbiasen(&mut self) -> HMICBIASEN_W<MICBIAS_SPEC> {
        HMICBIASEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - When this bit is 1and AUTOPLEN is 0, the MICDET is pulled down to GND."]
    #[inline(always)]
    #[must_use]
    pub fn micdetpl(&mut self) -> MICDETPL_W<MICBIAS_SPEC> {
        MICDETPL_W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable the function to auto pull low MICDET when jack removal"]
    #[inline(always)]
    #[must_use]
    pub fn autoplen(&mut self) -> AUTOPLEN_W<MICBIAS_SPEC> {
        AUTOPLEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - MIC Detect Mode"]
    #[inline(always)]
    #[must_use]
    pub fn det_mode(&mut self) -> DET_MODE_W<MICBIAS_SPEC> {
        DET_MODE_W::new(self, 18)
    }
    #[doc = "Bit 19 - When this bit is 0, HBIAS MICADC is controlled by registor"]
    #[inline(always)]
    #[must_use]
    pub fn popfree(&mut self) -> POPFREE_W<MICBIAS_SPEC> {
        POPFREE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Microphone detect ADC enable"]
    #[inline(always)]
    #[must_use]
    pub fn micadcen(&mut self) -> MICADCEN_W<MICBIAS_SPEC> {
        MICADCEN_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - Select the delay time to pull low the micdet when jack removal"]
    #[inline(always)]
    #[must_use]
    pub fn seldetadcdy(&mut self) -> SELDETADCDY_W<MICBIAS_SPEC> {
        SELDETADCDY_W::new(self, 21)
    }
    #[doc = "Bit 23 - Jack detect enable"]
    #[inline(always)]
    #[must_use]
    pub fn jackdeten(&mut self) -> JACKDETEN_W<MICBIAS_SPEC> {
        JACKDETEN_W::new(self, 23)
    }
    #[doc = "Bits 24:25 - Select the time to enable HBIAS before MICADC work"]
    #[inline(always)]
    #[must_use]
    pub fn seldetadcbf(&mut self) -> SELDETADCBF_W<MICBIAS_SPEC> {
        SELDETADCBF_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Select debounce time when jack removal"]
    #[inline(always)]
    #[must_use]
    pub fn seldetadcdb(&mut self) -> SELDETADCDB_W<MICBIAS_SPEC> {
        SELDETADCDB_W::new(self, 26)
    }
    #[doc = "Bits 28:30 - Select sample interval of the ADC sample\\\\ 2 ^ (SELDETADCFS + 1) ms"]
    #[inline(always)]
    #[must_use]
    pub fn seldetadcfs(&mut self) -> SELDETADCFS_W<MICBIAS_SPEC> {
        SELDETADCFS_W::new(self, 28)
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
#[doc = "MICBIAS Analog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`micbias::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`micbias::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MICBIAS_SPEC;
impl crate::RegisterSpec for MICBIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`micbias::R`](R) reader structure"]
impl crate::Readable for MICBIAS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`micbias::W`](W) writer structure"]
impl crate::Writable for MICBIAS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets micbias to value 0x4000_3030"]
impl crate::Resettable for MICBIAS_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_3030;
}
