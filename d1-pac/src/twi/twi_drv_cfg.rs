#[doc = "Register `twi_drv_cfg` reader"]
pub type R = crate::R<TWI_DRV_CFG_SPEC>;
#[doc = "Register `twi_drv_cfg` writer"]
pub type W = crate::W<TWI_DRV_CFG_SPEC>;
#[doc = "Field `packet_cnt` reader - "]
pub type PACKET_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `packet_cnt` writer - "]
pub type PACKET_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `pkt_interval` reader - "]
pub type PKT_INTERVAL_R = crate::FieldReader<u16>;
#[doc = "Field `pkt_interval` writer - "]
pub type PKT_INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn packet_cnt(&self) -> PACKET_CNT_R {
        PACKET_CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn pkt_interval(&self) -> PKT_INTERVAL_R {
        PKT_INTERVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn packet_cnt(&mut self) -> PACKET_CNT_W<TWI_DRV_CFG_SPEC> {
        PACKET_CNT_W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_interval(&mut self) -> PKT_INTERVAL_W<TWI_DRV_CFG_SPEC> {
        PKT_INTERVAL_W::new(self, 16)
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
#[doc = "TWI_DRV Transmission Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_drv_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_drv_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_DRV_CFG_SPEC;
impl crate::RegisterSpec for TWI_DRV_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_drv_cfg::R`](R) reader structure"]
impl crate::Readable for TWI_DRV_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_drv_cfg::W`](W) writer structure"]
impl crate::Writable for TWI_DRV_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_drv_cfg to value 0"]
impl crate::Resettable for TWI_DRV_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
