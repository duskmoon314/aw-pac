#[doc = "Register `asrcmancfg` reader"]
pub type R = crate::R<ASRCMANCFG_SPEC>;
#[doc = "Register `asrcmancfg` writer"]
pub type W = crate::W<ASRCMANCFG_SPEC>;
#[doc = "Field `asrc_ratio_value_manual_cfg` reader - ASRD Ration Value Manual Configure"]
pub type ASRC_RATIO_VALUE_MANUAL_CFG_R = crate::FieldReader<u32>;
#[doc = "Field `asrc_ratio_value_manual_cfg` writer - ASRD Ration Value Manual Configure"]
pub type ASRC_RATIO_VALUE_MANUAL_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `asrc_ratio_manual_en` reader - Manual Configuration of ASRC Ratio Enable"]
pub type ASRC_RATIO_MANUAL_EN_R = crate::BitReader<ASRC_RATIO_MANUAL_EN_A>;
#[doc = "Manual Configuration of ASRC Ratio Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASRC_RATIO_MANUAL_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ASRC_RATIO_MANUAL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_RATIO_MANUAL_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ASRC_RATIO_MANUAL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASRC_RATIO_MANUAL_EN_A {
        match self.bits {
            false => ASRC_RATIO_MANUAL_EN_A::DISABLE,
            true => ASRC_RATIO_MANUAL_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ASRC_RATIO_MANUAL_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ASRC_RATIO_MANUAL_EN_A::ENABLE
    }
}
#[doc = "Field `asrc_ratio_manual_en` writer - Manual Configuration of ASRC Ratio Enable"]
pub type ASRC_RATIO_MANUAL_EN_W<'a, REG> = crate::BitWriter<'a, REG, ASRC_RATIO_MANUAL_EN_A>;
impl<'a, REG> ASRC_RATIO_MANUAL_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ASRC_RATIO_MANUAL_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ASRC_RATIO_MANUAL_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:25 - ASRD Ration Value Manual Configure"]
    #[inline(always)]
    pub fn asrc_ratio_value_manual_cfg(&self) -> ASRC_RATIO_VALUE_MANUAL_CFG_R {
        ASRC_RATIO_VALUE_MANUAL_CFG_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 31 - Manual Configuration of ASRC Ratio Enable"]
    #[inline(always)]
    pub fn asrc_ratio_manual_en(&self) -> ASRC_RATIO_MANUAL_EN_R {
        ASRC_RATIO_MANUAL_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - ASRD Ration Value Manual Configure"]
    #[inline(always)]
    #[must_use]
    pub fn asrc_ratio_value_manual_cfg(
        &mut self,
    ) -> ASRC_RATIO_VALUE_MANUAL_CFG_W<ASRCMANCFG_SPEC> {
        ASRC_RATIO_VALUE_MANUAL_CFG_W::new(self, 0)
    }
    #[doc = "Bit 31 - Manual Configuration of ASRC Ratio Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asrc_ratio_manual_en(&mut self) -> ASRC_RATIO_MANUAL_EN_W<ASRCMANCFG_SPEC> {
        ASRC_RATIO_MANUAL_EN_W::new(self, 31)
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
#[doc = "ASRC Manual Ratio Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asrcmancfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asrcmancfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASRCMANCFG_SPEC;
impl crate::RegisterSpec for ASRCMANCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asrcmancfg::R`](R) reader structure"]
impl crate::Readable for ASRCMANCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asrcmancfg::W`](W) writer structure"]
impl crate::Writable for ASRCMANCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets asrcmancfg to value 0"]
impl crate::Resettable for ASRCMANCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
