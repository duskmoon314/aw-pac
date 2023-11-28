#[doc = "Register `emac_mii_data` reader"]
pub type R = crate::R<EMAC_MII_DATA_SPEC>;
#[doc = "Register `emac_mii_data` writer"]
pub type W = crate::W<EMAC_MII_DATA_SPEC>;
#[doc = "Field `mii_data` reader - "]
pub type MII_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `mii_data` writer - "]
pub type MII_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mii_data(&self) -> MII_DATA_R {
        MII_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn mii_data(&mut self) -> MII_DATA_W<EMAC_MII_DATA_SPEC> {
        MII_DATA_W::new(self, 0)
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
#[doc = "EMAC Management Interface Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_mii_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_mii_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_MII_DATA_SPEC;
impl crate::RegisterSpec for EMAC_MII_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_mii_data::R`](R) reader structure"]
impl crate::Readable for EMAC_MII_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emac_mii_data::W`](W) writer structure"]
impl crate::Writable for EMAC_MII_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_mii_data to value 0"]
impl crate::Resettable for EMAC_MII_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
