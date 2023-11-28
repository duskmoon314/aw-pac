#[doc = "Register `fsout_cfg` reader"]
pub type R = crate::R<FSOUT_CFG_SPEC>;
#[doc = "Register `fsout_cfg` writer"]
pub type W = crate::W<FSOUT_CFG_SPEC>;
#[doc = "Field `fsout_gate` reader - fsout Clock Gate Enable Control"]
pub type FSOUT_GATE_R = crate::BitReader<FSOUT_GATE_A>;
#[doc = "fsout Clock Gate Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSOUT_GATE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FSOUT_GATE_A> for bool {
    #[inline(always)]
    fn from(variant: FSOUT_GATE_A) -> Self {
        variant as u8 != 0
    }
}
impl FSOUT_GATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSOUT_GATE_A {
        match self.bits {
            false => FSOUT_GATE_A::DISABLE,
            true => FSOUT_GATE_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FSOUT_GATE_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FSOUT_GATE_A::ENABLE
    }
}
#[doc = "Field `fsout_gate` writer - fsout Clock Gate Enable Control"]
pub type FSOUT_GATE_W<'a, REG> = crate::BitWriter<'a, REG, FSOUT_GATE_A>;
impl<'a, REG> FSOUT_GATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FSOUT_GATE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FSOUT_GATE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 20 - fsout Clock Gate Enable Control"]
    #[inline(always)]
    pub fn fsout_gate(&self) -> FSOUT_GATE_R {
        FSOUT_GATE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - fsout Clock Gate Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn fsout_gate(&mut self) -> FSOUT_GATE_W<FSOUT_CFG_SPEC> {
        FSOUT_GATE_W::new(self, 20)
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
#[doc = "ASRC Out Sample Rate Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsout_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsout_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSOUT_CFG_SPEC;
impl crate::RegisterSpec for FSOUT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsout_cfg::R`](R) reader structure"]
impl crate::Readable for FSOUT_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fsout_cfg::W`](W) writer structure"]
impl crate::Writable for FSOUT_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fsout_cfg to value 0"]
impl crate::Resettable for FSOUT_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
