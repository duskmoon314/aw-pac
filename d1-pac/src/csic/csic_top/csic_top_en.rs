#[doc = "Register `csic_top_en` reader"]
pub type R = crate::R<CSIC_TOP_EN_SPEC>;
#[doc = "Register `csic_top_en` writer"]
pub type W = crate::W<CSIC_TOP_EN_SPEC>;
#[doc = "Field `csic_top_en` reader - "]
pub type CSIC_TOP_EN_R = crate::BitReader<CSIC_TOP_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSIC_TOP_EN_A {
    #[doc = "0: Reset and disable the CSIC module"]
    RESET_DISABLE = 0,
    #[doc = "1: Enable the CSIC module"]
    ENABLE = 1,
}
impl From<CSIC_TOP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CSIC_TOP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CSIC_TOP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSIC_TOP_EN_A {
        match self.bits {
            false => CSIC_TOP_EN_A::RESET_DISABLE,
            true => CSIC_TOP_EN_A::ENABLE,
        }
    }
    #[doc = "Reset and disable the CSIC module"]
    #[inline(always)]
    pub fn is_reset_disable(&self) -> bool {
        *self == CSIC_TOP_EN_A::RESET_DISABLE
    }
    #[doc = "Enable the CSIC module"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CSIC_TOP_EN_A::ENABLE
    }
}
#[doc = "Field `csic_top_en` writer - "]
pub type CSIC_TOP_EN_W<'a, REG> = crate::BitWriter<'a, REG, CSIC_TOP_EN_A>;
impl<'a, REG> CSIC_TOP_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset and disable the CSIC module"]
    #[inline(always)]
    pub fn reset_disable(self) -> &'a mut crate::W<REG> {
        self.variant(CSIC_TOP_EN_A::RESET_DISABLE)
    }
    #[doc = "Enable the CSIC module"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CSIC_TOP_EN_A::ENABLE)
    }
}
#[doc = "Field `bist_mode_en` reader - "]
pub type BIST_MODE_EN_R = crate::BitReader<BIST_MODE_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIST_MODE_EN_A {
    #[doc = "0: Closed"]
    C_LOSED = 0,
    #[doc = "1: EN BIST TEST"]
    EN = 1,
}
impl From<BIST_MODE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BIST_MODE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BIST_MODE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BIST_MODE_EN_A {
        match self.bits {
            false => BIST_MODE_EN_A::C_LOSED,
            true => BIST_MODE_EN_A::EN,
        }
    }
    #[doc = "Closed"]
    #[inline(always)]
    pub fn is_c_losed(&self) -> bool {
        *self == BIST_MODE_EN_A::C_LOSED
    }
    #[doc = "EN BIST TEST"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BIST_MODE_EN_A::EN
    }
}
#[doc = "Field `bist_mode_en` writer - "]
pub type BIST_MODE_EN_W<'a, REG> = crate::BitWriter<'a, REG, BIST_MODE_EN_A>;
impl<'a, REG> BIST_MODE_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Closed"]
    #[inline(always)]
    pub fn c_losed(self) -> &'a mut crate::W<REG> {
        self.variant(BIST_MODE_EN_A::C_LOSED)
    }
    #[doc = "EN BIST TEST"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(BIST_MODE_EN_A::EN)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn csic_top_en(&self) -> CSIC_TOP_EN_R {
        CSIC_TOP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bist_mode_en(&self) -> BIST_MODE_EN_R {
        BIST_MODE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn csic_top_en(&mut self) -> CSIC_TOP_EN_W<CSIC_TOP_EN_SPEC> {
        CSIC_TOP_EN_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn bist_mode_en(&mut self) -> BIST_MODE_EN_W<CSIC_TOP_EN_SPEC> {
        BIST_MODE_EN_W::new(self, 2)
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
#[doc = "CSIC TOP Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_top_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_top_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_TOP_EN_SPEC;
impl crate::RegisterSpec for CSIC_TOP_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_top_en::R`](R) reader structure"]
impl crate::Readable for CSIC_TOP_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_top_en::W`](W) writer structure"]
impl crate::Writable for CSIC_TOP_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_top_en to value 0"]
impl crate::Resettable for CSIC_TOP_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
