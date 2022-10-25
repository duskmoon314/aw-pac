#[doc = "Register `twi_drv_cfg` reader"]
pub struct R(crate::R<TWI_DRV_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_DRV_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_DRV_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_DRV_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `twi_drv_cfg` writer"]
pub struct W(crate::W<TWI_DRV_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_DRV_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TWI_DRV_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_DRV_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `packet_cnt` reader - "]
pub type PACKET_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `packet_cnt` writer - "]
pub type PACKET_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TWI_DRV_CFG_SPEC, u16, u16, 16, O>;
#[doc = "Field `pkt_interval` reader - "]
pub type PKT_INTERVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pkt_interval` writer - "]
pub type PKT_INTERVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TWI_DRV_CFG_SPEC, u16, u16, 16, O>;
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
    pub fn packet_cnt(&mut self) -> PACKET_CNT_W<0> {
        PACKET_CNT_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_interval(&mut self) -> PKT_INTERVAL_W<16> {
        PKT_INTERVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI_DRV Transmission Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_drv_cfg](index.html) module"]
pub struct TWI_DRV_CFG_SPEC;
impl crate::RegisterSpec for TWI_DRV_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_drv_cfg::R](R) reader structure"]
impl crate::Readable for TWI_DRV_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_drv_cfg::W](W) writer structure"]
impl crate::Writable for TWI_DRV_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_drv_cfg to value 0"]
impl crate::Resettable for TWI_DRV_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
