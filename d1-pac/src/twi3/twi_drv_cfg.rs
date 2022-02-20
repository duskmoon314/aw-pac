#[doc = "Register `TWI_DRV_CFG` reader"]
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
#[doc = "Register `TWI_DRV_CFG` writer"]
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
#[doc = "Field `pkt_interval` reader - "]
pub struct PKT_INTERVAL_R(crate::FieldReader<u16, u16>);
impl PKT_INTERVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PKT_INTERVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKT_INTERVAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pkt_interval` writer - "]
pub struct PKT_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PKT_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `packet_cnt` reader - "]
pub struct PACKET_CNT_R(crate::FieldReader<u16, u16>);
impl PACKET_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PACKET_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PACKET_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `packet_cnt` writer - "]
pub struct PACKET_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKET_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn pkt_interval(&self) -> PKT_INTERVAL_R {
        PKT_INTERVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn packet_cnt(&self) -> PACKET_CNT_R {
        PACKET_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn pkt_interval(&mut self) -> PKT_INTERVAL_W {
        PKT_INTERVAL_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn packet_cnt(&mut self) -> PACKET_CNT_W {
        PACKET_CNT_W { w: self }
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
}
#[doc = "`reset()` method sets TWI_DRV_CFG to value 0"]
impl crate::Resettable for TWI_DRV_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
