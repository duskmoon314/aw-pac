#[doc = "Register `emac_addr_high0` reader"]
pub type R = crate::R<EMAC_ADDR_HIGH0_SPEC>;
#[doc = "Register `emac_addr_high0` writer"]
pub type W = crate::W<EMAC_ADDR_HIGH0_SPEC>;
#[doc = "Field `mac_addr_high0` reader - "]
pub type MAC_ADDR_HIGH0_R = crate::FieldReader<u16>;
#[doc = "Field `mac_addr_high0` writer - "]
pub type MAC_ADDR_HIGH0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mac_addr_high0(&self) -> MAC_ADDR_HIGH0_R {
        MAC_ADDR_HIGH0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn mac_addr_high0(&mut self) -> MAC_ADDR_HIGH0_W<EMAC_ADDR_HIGH0_SPEC> {
        MAC_ADDR_HIGH0_W::new(self, 0)
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
#[doc = "EMAC MAC Address High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_addr_high0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_addr_high0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_ADDR_HIGH0_SPEC;
impl crate::RegisterSpec for EMAC_ADDR_HIGH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_addr_high0::R`](R) reader structure"]
impl crate::Readable for EMAC_ADDR_HIGH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emac_addr_high0::W`](W) writer structure"]
impl crate::Writable for EMAC_ADDR_HIGH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_addr_high0 to value 0"]
impl crate::Resettable for EMAC_ADDR_HIGH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
