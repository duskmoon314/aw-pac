#[doc = "Register `tve_dac_cfg3` reader"]
pub type R = crate::R<TVE_DAC_CFG3_SPEC>;
#[doc = "Register `tve_dac_cfg3` writer"]
pub type W = crate::W<TVE_DAC_CFG3_SPEC>;
#[doc = "Field `force_data_en` reader - "]
pub type FORCE_DATA_EN_R = crate::BitReader<FORCE_DATA_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_DATA_EN_A {
    #[doc = "0: DAC input data from TVE"]
    TVE = 0,
    #[doc = "1: DAC input data from FORCE_DATA_SET"]
    FORCE_DATA_SET = 1,
}
impl From<FORCE_DATA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_DATA_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCE_DATA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FORCE_DATA_EN_A {
        match self.bits {
            false => FORCE_DATA_EN_A::TVE,
            true => FORCE_DATA_EN_A::FORCE_DATA_SET,
        }
    }
    #[doc = "DAC input data from TVE"]
    #[inline(always)]
    pub fn is_tve(&self) -> bool {
        *self == FORCE_DATA_EN_A::TVE
    }
    #[doc = "DAC input data from FORCE_DATA_SET"]
    #[inline(always)]
    pub fn is_force_data_set(&self) -> bool {
        *self == FORCE_DATA_EN_A::FORCE_DATA_SET
    }
}
#[doc = "Field `force_data_en` writer - "]
pub type FORCE_DATA_EN_W<'a, REG> = crate::BitWriter<'a, REG, FORCE_DATA_EN_A>;
impl<'a, REG> FORCE_DATA_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC input data from TVE"]
    #[inline(always)]
    pub fn tve(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_DATA_EN_A::TVE)
    }
    #[doc = "DAC input data from FORCE_DATA_SET"]
    #[inline(always)]
    pub fn force_data_set(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_DATA_EN_A::FORCE_DATA_SET)
    }
}
#[doc = "Field `force_data_set` reader - Force DAC input data"]
pub type FORCE_DATA_SET_R = crate::FieldReader<u16>;
#[doc = "Field `force_data_set` writer - Force DAC input data"]
pub type FORCE_DATA_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn force_data_en(&self) -> FORCE_DATA_EN_R {
        FORCE_DATA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:25 - Force DAC input data"]
    #[inline(always)]
    pub fn force_data_set(&self) -> FORCE_DATA_SET_R {
        FORCE_DATA_SET_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn force_data_en(&mut self) -> FORCE_DATA_EN_W<TVE_DAC_CFG3_SPEC> {
        FORCE_DATA_EN_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Force DAC input data"]
    #[inline(always)]
    #[must_use]
    pub fn force_data_set(&mut self) -> FORCE_DATA_SET_W<TVE_DAC_CFG3_SPEC> {
        FORCE_DATA_SET_W::new(self, 16)
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
#[doc = "TV Encoder DAC CFG2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_dac_cfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_dac_cfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_DAC_CFG3_SPEC;
impl crate::RegisterSpec for TVE_DAC_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_dac_cfg3::R`](R) reader structure"]
impl crate::Readable for TVE_DAC_CFG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_dac_cfg3::W`](W) writer structure"]
impl crate::Writable for TVE_DAC_CFG3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_dac_cfg3 to value 0"]
impl crate::Resettable for TVE_DAC_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
