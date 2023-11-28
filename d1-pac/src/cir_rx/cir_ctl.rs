#[doc = "Register `cir_ctl` reader"]
pub type R = crate::R<CIR_CTL_SPEC>;
#[doc = "Register `cir_ctl` writer"]
pub type W = crate::W<CIR_CTL_SPEC>;
#[doc = "Field `gen` reader - Global Enable\n\nA disable on this bit overrides any other block or channel enables and flushes all FIFOs."]
pub type GEN_R = crate::BitReader<GEN_A>;
#[doc = "Global Enable\n\nA disable on this bit overrides any other block or channel enables and flushes all FIFOs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<GEN_A> for bool {
    #[inline(always)]
    fn from(variant: GEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GEN_A {
        match self.bits {
            false => GEN_A::DISABLE,
            true => GEN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GEN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GEN_A::ENABLE
    }
}
#[doc = "Field `gen` writer - Global Enable\n\nA disable on this bit overrides any other block or channel enables and flushes all FIFOs."]
pub type GEN_W<'a, REG> = crate::BitWriter<'a, REG, GEN_A>;
impl<'a, REG> GEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(GEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(GEN_A::ENABLE)
    }
}
#[doc = "Field `rxen` reader - Receiver Block Enable"]
pub type RXEN_R = crate::BitReader<RXEN_A>;
#[doc = "Receiver Block Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<RXEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXEN_A {
        match self.bits {
            false => RXEN_A::DISABLE,
            true => RXEN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXEN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXEN_A::ENABLE
    }
}
#[doc = "Field `rxen` writer - Receiver Block Enable"]
pub type RXEN_W<'a, REG> = crate::BitWriter<'a, REG, RXEN_A>;
impl<'a, REG> RXEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RXEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RXEN_A::ENABLE)
    }
}
#[doc = "Field `ciren` reader - CIR Enable"]
pub type CIREN_R = crate::FieldReader<CIREN_A>;
#[doc = "CIR Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CIREN_A {
    #[doc = "3: CIR mode enable"]
    ENABLE = 3,
}
impl From<CIREN_A> for u8 {
    #[inline(always)]
    fn from(variant: CIREN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CIREN_A {
    type Ux = u8;
}
impl CIREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CIREN_A> {
        match self.bits {
            3 => Some(CIREN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "CIR mode enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CIREN_A::ENABLE
    }
}
#[doc = "Field `ciren` writer - CIR Enable"]
pub type CIREN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CIREN_A>;
impl<'a, REG> CIREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CIR mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CIREN_A::ENABLE)
    }
}
#[doc = "Field `apam` reader - Active Pulse Accept Mode"]
pub type APAM_R = crate::FieldReader<APAM_A>;
#[doc = "Active Pulse Accept Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum APAM_A {
    #[doc = "0: Both positive and negative pulses are valid as a leading code"]
    BOTH_VALID = 0,
    #[doc = "2: Only negative pulse is valid as a leading code"]
    ONLY_NEGATIVE = 2,
    #[doc = "3: Only positive pulse is valid as a leading code"]
    ONLY_POSITIVE = 3,
}
impl From<APAM_A> for u8 {
    #[inline(always)]
    fn from(variant: APAM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for APAM_A {
    type Ux = u8;
}
impl APAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<APAM_A> {
        match self.bits {
            0 => Some(APAM_A::BOTH_VALID),
            2 => Some(APAM_A::ONLY_NEGATIVE),
            3 => Some(APAM_A::ONLY_POSITIVE),
            _ => None,
        }
    }
    #[doc = "Both positive and negative pulses are valid as a leading code"]
    #[inline(always)]
    pub fn is_both_valid(&self) -> bool {
        *self == APAM_A::BOTH_VALID
    }
    #[doc = "Only negative pulse is valid as a leading code"]
    #[inline(always)]
    pub fn is_only_negative(&self) -> bool {
        *self == APAM_A::ONLY_NEGATIVE
    }
    #[doc = "Only positive pulse is valid as a leading code"]
    #[inline(always)]
    pub fn is_only_positive(&self) -> bool {
        *self == APAM_A::ONLY_POSITIVE
    }
}
#[doc = "Field `apam` writer - Active Pulse Accept Mode"]
pub type APAM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, APAM_A>;
impl<'a, REG> APAM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Both positive and negative pulses are valid as a leading code"]
    #[inline(always)]
    pub fn both_valid(self) -> &'a mut crate::W<REG> {
        self.variant(APAM_A::BOTH_VALID)
    }
    #[doc = "Only negative pulse is valid as a leading code"]
    #[inline(always)]
    pub fn only_negative(self) -> &'a mut crate::W<REG> {
        self.variant(APAM_A::ONLY_NEGATIVE)
    }
    #[doc = "Only positive pulse is valid as a leading code"]
    #[inline(always)]
    pub fn only_positive(self) -> &'a mut crate::W<REG> {
        self.variant(APAM_A::ONLY_POSITIVE)
    }
}
impl R {
    #[doc = "Bit 0 - Global Enable\n\nA disable on this bit overrides any other block or channel enables and flushes all FIFOs."]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver Block Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - CIR Enable"]
    #[inline(always)]
    pub fn ciren(&self) -> CIREN_R {
        CIREN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Active Pulse Accept Mode"]
    #[inline(always)]
    pub fn apam(&self) -> APAM_R {
        APAM_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Global Enable\n\nA disable on this bit overrides any other block or channel enables and flushes all FIFOs."]
    #[inline(always)]
    #[must_use]
    pub fn gen(&mut self) -> GEN_W<CIR_CTL_SPEC> {
        GEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver Block Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<CIR_CTL_SPEC> {
        RXEN_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - CIR Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ciren(&mut self) -> CIREN_W<CIR_CTL_SPEC> {
        CIREN_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Active Pulse Accept Mode"]
    #[inline(always)]
    #[must_use]
    pub fn apam(&mut self) -> APAM_W<CIR_CTL_SPEC> {
        APAM_W::new(self, 6)
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
#[doc = "CIR Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIR_CTL_SPEC;
impl crate::RegisterSpec for CIR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir_ctl::R`](R) reader structure"]
impl crate::Readable for CIR_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cir_ctl::W`](W) writer structure"]
impl crate::Writable for CIR_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_ctl to value 0"]
impl crate::Resettable for CIR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
