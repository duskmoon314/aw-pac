#[doc = "Register `EMAC_ADDR_HIGH0` reader"]
pub struct R(crate::R<EMAC_ADDR_HIGH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_ADDR_HIGH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_ADDR_HIGH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_ADDR_HIGH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMAC_ADDR_HIGH0` writer"]
pub struct W(crate::W<EMAC_ADDR_HIGH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_ADDR_HIGH0_SPEC>;
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
impl From<crate::W<EMAC_ADDR_HIGH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_ADDR_HIGH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAC_ADDR_HIGH0` reader - "]
pub struct MAC_ADDR_HIGH0_R(crate::FieldReader<u16, u16>);
impl MAC_ADDR_HIGH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MAC_ADDR_HIGH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAC_ADDR_HIGH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAC_ADDR_HIGH0` writer - "]
pub struct MAC_ADDR_HIGH0_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_ADDR_HIGH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
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
    pub fn mac_addr_high0(&mut self) -> MAC_ADDR_HIGH0_W {
        MAC_ADDR_HIGH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC MAC Address High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_addr_high0](index.html) module"]
pub struct EMAC_ADDR_HIGH0_SPEC;
impl crate::RegisterSpec for EMAC_ADDR_HIGH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_addr_high0::R](R) reader structure"]
impl crate::Readable for EMAC_ADDR_HIGH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_addr_high0::W](W) writer structure"]
impl crate::Writable for EMAC_ADDR_HIGH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMAC_ADDR_HIGH0 to value 0"]
impl crate::Resettable for EMAC_ADDR_HIGH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
