#[doc = "Register `fsin_extcfg` reader"]
pub type R = crate::R<FSIN_EXTCFG_SPEC>;
#[doc = "Register `fsin_extcfg` writer"]
pub type W = crate::W<FSIN_EXTCFG_SPEC>;
#[doc = "Field `cyclenum` reader - The Cycle Number of Pulse Extend.\n\nThe cycle is BCLK and is at least 1."]
pub type CYCLENUM_R = crate::FieldReader<u16>;
#[doc = "Field `cyclenum` writer - The Cycle Number of Pulse Extend.\n\nThe cycle is BCLK and is at least 1."]
pub type CYCLENUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `extend` reader - Extend the bit when using ASRC."]
pub type EXTEND_R = crate::BitReader<EXTEND_A>;
#[doc = "Extend the bit when using ASRC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTEND_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<EXTEND_A> for bool {
    #[inline(always)]
    fn from(variant: EXTEND_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTEND_A {
        match self.bits {
            false => EXTEND_A::DISABLE,
            true => EXTEND_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EXTEND_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EXTEND_A::ENABLE
    }
}
#[doc = "Field `extend` writer - Extend the bit when using ASRC."]
pub type EXTEND_W<'a, REG> = crate::BitWriter<'a, REG, EXTEND_A>;
impl<'a, REG> EXTEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEND_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEND_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:15 - The Cycle Number of Pulse Extend.\n\nThe cycle is BCLK and is at least 1."]
    #[inline(always)]
    pub fn cyclenum(&self) -> CYCLENUM_R {
        CYCLENUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Extend the bit when using ASRC."]
    #[inline(always)]
    pub fn extend(&self) -> EXTEND_R {
        EXTEND_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - The Cycle Number of Pulse Extend.\n\nThe cycle is BCLK and is at least 1."]
    #[inline(always)]
    #[must_use]
    pub fn cyclenum(&mut self) -> CYCLENUM_W<FSIN_EXTCFG_SPEC> {
        CYCLENUM_W::new(self, 0)
    }
    #[doc = "Bit 16 - Extend the bit when using ASRC."]
    #[inline(always)]
    #[must_use]
    pub fn extend(&mut self) -> EXTEND_W<FSIN_EXTCFG_SPEC> {
        EXTEND_W::new(self, 16)
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
#[doc = "ASRC Input Sample Pulse Extend Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsin_extcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsin_extcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSIN_EXTCFG_SPEC;
impl crate::RegisterSpec for FSIN_EXTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsin_extcfg::R`](R) reader structure"]
impl crate::Readable for FSIN_EXTCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fsin_extcfg::W`](W) writer structure"]
impl crate::Writable for FSIN_EXTCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fsin_extcfg to value 0"]
impl crate::Resettable for FSIN_EXTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
