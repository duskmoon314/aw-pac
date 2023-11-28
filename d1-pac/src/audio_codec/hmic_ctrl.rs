#[doc = "Register `hmic_ctrl` reader"]
pub type R = crate::R<HMIC_CTRL_SPEC>;
#[doc = "Register `hmic_ctrl` writer"]
pub type W = crate::W<HMIC_CTRL_SPEC>;
#[doc = "Field `mic_det_irq_en` reader - MIC Detect Interrupt Set"]
pub type MIC_DET_IRQ_EN_R = crate::BitReader<MIC_DET_IRQ_EN_A>;
#[doc = "MIC Detect Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIC_DET_IRQ_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<MIC_DET_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MIC_DET_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl MIC_DET_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIC_DET_IRQ_EN_A {
        match self.bits {
            false => MIC_DET_IRQ_EN_A::DISABLED,
            true => MIC_DET_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MIC_DET_IRQ_EN_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MIC_DET_IRQ_EN_A::ENABLED
    }
}
#[doc = "Field `mic_det_irq_en` writer - MIC Detect Interrupt Set"]
pub type MIC_DET_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, MIC_DET_IRQ_EN_A>;
impl<'a, REG> MIC_DET_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MIC_DET_IRQ_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MIC_DET_IRQ_EN_A::ENABLED)
    }
}
#[doc = "Field `jack_in_irq_en` reader - MIC Detect Interrupt Set"]
pub type JACK_IN_IRQ_EN_R = crate::BitReader<JACK_IN_IRQ_EN_A>;
#[doc = "MIC Detect Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JACK_IN_IRQ_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<JACK_IN_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: JACK_IN_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl JACK_IN_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JACK_IN_IRQ_EN_A {
        match self.bits {
            false => JACK_IN_IRQ_EN_A::DISABLED,
            true => JACK_IN_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JACK_IN_IRQ_EN_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JACK_IN_IRQ_EN_A::ENABLED
    }
}
#[doc = "Field `jack_in_irq_en` writer - MIC Detect Interrupt Set"]
pub type JACK_IN_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, JACK_IN_IRQ_EN_A>;
impl<'a, REG> JACK_IN_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JACK_IN_IRQ_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JACK_IN_IRQ_EN_A::ENABLED)
    }
}
#[doc = "Field `jack_out_irq_en` reader - MIC Detect Interrupt Set"]
pub type JACK_OUT_IRQ_EN_R = crate::BitReader<JACK_OUT_IRQ_EN_A>;
#[doc = "MIC Detect Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JACK_OUT_IRQ_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<JACK_OUT_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: JACK_OUT_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl JACK_OUT_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JACK_OUT_IRQ_EN_A {
        match self.bits {
            false => JACK_OUT_IRQ_EN_A::DISABLED,
            true => JACK_OUT_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JACK_OUT_IRQ_EN_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JACK_OUT_IRQ_EN_A::ENABLED
    }
}
#[doc = "Field `jack_out_irq_en` writer - MIC Detect Interrupt Set"]
pub type JACK_OUT_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, JACK_OUT_IRQ_EN_A>;
impl<'a, REG> JACK_OUT_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JACK_OUT_IRQ_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JACK_OUT_IRQ_EN_A::ENABLED)
    }
}
#[doc = "Field `mdata_threshold_debounce` reader - MDATA Threshold Debounce"]
pub type MDATA_THRESHOLD_DEBOUNCE_R = crate::FieldReader;
#[doc = "Field `mdata_threshold_debounce` writer - MDATA Threshold Debounce"]
pub type MDATA_THRESHOLD_DEBOUNCE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `hmic_n` reader - Debounce when earphone plug in or pull out 125 ms to 2 s"]
pub type HMIC_N_R = crate::FieldReader;
#[doc = "Field `hmic_n` writer - Debounce when earphone plug in or pull out 125 ms to 2 s"]
pub type HMIC_N_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hmic_m` reader - Debounce when the MIC Key down or up.\n\n(HMIC_M + 1) sample data"]
pub type HMIC_M_R = crate::FieldReader;
#[doc = "Field `hmic_m` writer - Debounce when the MIC Key down or up.\n\n(HMIC_M + 1) sample data"]
pub type HMIC_M_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hmic_sf` reader - HMIC Smooth Filter setting"]
pub type HMIC_SF_R = crate::FieldReader<HMIC_SF_A>;
#[doc = "HMIC Smooth Filter setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HMIC_SF_A {
    #[doc = "0: bypass"]
    BYPASS = 0,
    #[doc = "1: (x1+x2)/2"]
    MEAN_2 = 1,
    #[doc = "2: (x1+x2+x3+x4)/4"]
    MEAN_4 = 2,
    #[doc = "3: (x1+x2+x3+x4+x5+x6+x7+x8)/8"]
    MEAN_8 = 3,
}
impl From<HMIC_SF_A> for u8 {
    #[inline(always)]
    fn from(variant: HMIC_SF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HMIC_SF_A {
    type Ux = u8;
}
impl HMIC_SF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HMIC_SF_A {
        match self.bits {
            0 => HMIC_SF_A::BYPASS,
            1 => HMIC_SF_A::MEAN_2,
            2 => HMIC_SF_A::MEAN_4,
            3 => HMIC_SF_A::MEAN_8,
            _ => unreachable!(),
        }
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == HMIC_SF_A::BYPASS
    }
    #[doc = "(x1+x2)/2"]
    #[inline(always)]
    pub fn is_mean_2(&self) -> bool {
        *self == HMIC_SF_A::MEAN_2
    }
    #[doc = "(x1+x2+x3+x4)/4"]
    #[inline(always)]
    pub fn is_mean_4(&self) -> bool {
        *self == HMIC_SF_A::MEAN_4
    }
    #[doc = "(x1+x2+x3+x4+x5+x6+x7+x8)/8"]
    #[inline(always)]
    pub fn is_mean_8(&self) -> bool {
        *self == HMIC_SF_A::MEAN_8
    }
}
#[doc = "Field `hmic_sf` writer - HMIC Smooth Filter setting"]
pub type HMIC_SF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, HMIC_SF_A>;
impl<'a, REG> HMIC_SF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(HMIC_SF_A::BYPASS)
    }
    #[doc = "(x1+x2)/2"]
    #[inline(always)]
    pub fn mean_2(self) -> &'a mut crate::W<REG> {
        self.variant(HMIC_SF_A::MEAN_2)
    }
    #[doc = "(x1+x2+x3+x4)/4"]
    #[inline(always)]
    pub fn mean_4(self) -> &'a mut crate::W<REG> {
        self.variant(HMIC_SF_A::MEAN_4)
    }
    #[doc = "(x1+x2+x3+x4+x5+x6+x7+x8)/8"]
    #[inline(always)]
    pub fn mean_8(self) -> &'a mut crate::W<REG> {
        self.variant(HMIC_SF_A::MEAN_8)
    }
}
#[doc = "Field `mdata_threshold` reader - MIC DET EN Threshold Value"]
pub type MDATA_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `mdata_threshold` writer - MIC DET EN Threshold Value"]
pub type MDATA_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hmic_sample_select` reader - Down Sample Setting Select"]
pub type HMIC_SAMPLE_SELECT_R = crate::FieldReader<HMIC_SAMPLE_SELECT_A>;
#[doc = "Down Sample Setting Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HMIC_SAMPLE_SELECT_A {
    #[doc = "0: Down by 1, 128 Hz"]
    BY_1 = 0,
    #[doc = "1: Down by 2, 64 Hz"]
    BY_2 = 1,
    #[doc = "2: Down by 4, 32 Hz"]
    BY_4 = 2,
    #[doc = "3: Down by 8, 16 Hz"]
    BY_8 = 3,
}
impl From<HMIC_SAMPLE_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: HMIC_SAMPLE_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HMIC_SAMPLE_SELECT_A {
    type Ux = u8;
}
impl HMIC_SAMPLE_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HMIC_SAMPLE_SELECT_A {
        match self.bits {
            0 => HMIC_SAMPLE_SELECT_A::BY_1,
            1 => HMIC_SAMPLE_SELECT_A::BY_2,
            2 => HMIC_SAMPLE_SELECT_A::BY_4,
            3 => HMIC_SAMPLE_SELECT_A::BY_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Down by 1, 128 Hz"]
    #[inline(always)]
    pub fn is_by_1(&self) -> bool {
        *self == HMIC_SAMPLE_SELECT_A::BY_1
    }
    #[doc = "Down by 2, 64 Hz"]
    #[inline(always)]
    pub fn is_by_2(&self) -> bool {
        *self == HMIC_SAMPLE_SELECT_A::BY_2
    }
    #[doc = "Down by 4, 32 Hz"]
    #[inline(always)]
    pub fn is_by_4(&self) -> bool {
        *self == HMIC_SAMPLE_SELECT_A::BY_4
    }
    #[doc = "Down by 8, 16 Hz"]
    #[inline(always)]
    pub fn is_by_8(&self) -> bool {
        *self == HMIC_SAMPLE_SELECT_A::BY_8
    }
}
#[doc = "Field `hmic_sample_select` writer - Down Sample Setting Select"]
pub type HMIC_SAMPLE_SELECT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, HMIC_SAMPLE_SELECT_A>;
impl<'a, REG> HMIC_SAMPLE_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Down by 1, 128 Hz"]
    #[inline(always)]
    pub fn by_1(self) -> &'a mut crate::W<REG> {
        self.variant(HMIC_SAMPLE_SELECT_A::BY_1)
    }
    #[doc = "Down by 2, 64 Hz"]
    #[inline(always)]
    pub fn by_2(self) -> &'a mut crate::W<REG> {
        self.variant(HMIC_SAMPLE_SELECT_A::BY_2)
    }
    #[doc = "Down by 4, 32 Hz"]
    #[inline(always)]
    pub fn by_4(self) -> &'a mut crate::W<REG> {
        self.variant(HMIC_SAMPLE_SELECT_A::BY_4)
    }
    #[doc = "Down by 8, 16 Hz"]
    #[inline(always)]
    pub fn by_8(self) -> &'a mut crate::W<REG> {
        self.variant(HMIC_SAMPLE_SELECT_A::BY_8)
    }
}
impl R {
    #[doc = "Bit 0 - MIC Detect Interrupt Set"]
    #[inline(always)]
    pub fn mic_det_irq_en(&self) -> MIC_DET_IRQ_EN_R {
        MIC_DET_IRQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MIC Detect Interrupt Set"]
    #[inline(always)]
    pub fn jack_in_irq_en(&self) -> JACK_IN_IRQ_EN_R {
        JACK_IN_IRQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MIC Detect Interrupt Set"]
    #[inline(always)]
    pub fn jack_out_irq_en(&self) -> JACK_OUT_IRQ_EN_R {
        JACK_OUT_IRQ_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - MDATA Threshold Debounce"]
    #[inline(always)]
    pub fn mdata_threshold_debounce(&self) -> MDATA_THRESHOLD_DEBOUNCE_R {
        MDATA_THRESHOLD_DEBOUNCE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:9 - Debounce when earphone plug in or pull out 125 ms to 2 s"]
    #[inline(always)]
    pub fn hmic_n(&self) -> HMIC_N_R {
        HMIC_N_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:13 - Debounce when the MIC Key down or up.\n\n(HMIC_M + 1) sample data"]
    #[inline(always)]
    pub fn hmic_m(&self) -> HMIC_M_R {
        HMIC_M_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - HMIC Smooth Filter setting"]
    #[inline(always)]
    pub fn hmic_sf(&self) -> HMIC_SF_R {
        HMIC_SF_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:20 - MIC DET EN Threshold Value"]
    #[inline(always)]
    pub fn mdata_threshold(&self) -> MDATA_THRESHOLD_R {
        MDATA_THRESHOLD_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Down Sample Setting Select"]
    #[inline(always)]
    pub fn hmic_sample_select(&self) -> HMIC_SAMPLE_SELECT_R {
        HMIC_SAMPLE_SELECT_R::new(((self.bits >> 21) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MIC Detect Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn mic_det_irq_en(&mut self) -> MIC_DET_IRQ_EN_W<HMIC_CTRL_SPEC> {
        MIC_DET_IRQ_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - MIC Detect Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn jack_in_irq_en(&mut self) -> JACK_IN_IRQ_EN_W<HMIC_CTRL_SPEC> {
        JACK_IN_IRQ_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - MIC Detect Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn jack_out_irq_en(&mut self) -> JACK_OUT_IRQ_EN_W<HMIC_CTRL_SPEC> {
        JACK_OUT_IRQ_EN_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - MDATA Threshold Debounce"]
    #[inline(always)]
    #[must_use]
    pub fn mdata_threshold_debounce(&mut self) -> MDATA_THRESHOLD_DEBOUNCE_W<HMIC_CTRL_SPEC> {
        MDATA_THRESHOLD_DEBOUNCE_W::new(self, 3)
    }
    #[doc = "Bits 6:9 - Debounce when earphone plug in or pull out 125 ms to 2 s"]
    #[inline(always)]
    #[must_use]
    pub fn hmic_n(&mut self) -> HMIC_N_W<HMIC_CTRL_SPEC> {
        HMIC_N_W::new(self, 6)
    }
    #[doc = "Bits 10:13 - Debounce when the MIC Key down or up.\n\n(HMIC_M + 1) sample data"]
    #[inline(always)]
    #[must_use]
    pub fn hmic_m(&mut self) -> HMIC_M_W<HMIC_CTRL_SPEC> {
        HMIC_M_W::new(self, 10)
    }
    #[doc = "Bits 14:15 - HMIC Smooth Filter setting"]
    #[inline(always)]
    #[must_use]
    pub fn hmic_sf(&mut self) -> HMIC_SF_W<HMIC_CTRL_SPEC> {
        HMIC_SF_W::new(self, 14)
    }
    #[doc = "Bits 16:20 - MIC DET EN Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn mdata_threshold(&mut self) -> MDATA_THRESHOLD_W<HMIC_CTRL_SPEC> {
        MDATA_THRESHOLD_W::new(self, 16)
    }
    #[doc = "Bits 21:22 - Down Sample Setting Select"]
    #[inline(always)]
    #[must_use]
    pub fn hmic_sample_select(&mut self) -> HMIC_SAMPLE_SELECT_W<HMIC_CTRL_SPEC> {
        HMIC_SAMPLE_SELECT_W::new(self, 21)
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
#[doc = "HMIC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hmic_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hmic_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HMIC_CTRL_SPEC;
impl crate::RegisterSpec for HMIC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hmic_ctrl::R`](R) reader structure"]
impl crate::Readable for HMIC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hmic_ctrl::W`](W) writer structure"]
impl crate::Writable for HMIC_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hmic_ctrl to value 0x08"]
impl crate::Resettable for HMIC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
