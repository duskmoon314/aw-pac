#[doc = "Register `USB_BGR` reader"]
pub struct R(crate::R<USB_BGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_BGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_BGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_BGR` writer"]
pub struct W(crate::W<USB_BGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_BGR_SPEC>;
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
impl From<crate::W<USB_BGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_BGR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Bus Gating Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_bgr](index.html) module"]
pub struct USB_BGR_SPEC;
impl crate::RegisterSpec for USB_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_bgr::R](R) reader structure"]
impl crate::Readable for USB_BGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_bgr::W](W) writer structure"]
impl crate::Writable for USB_BGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_BGR to value 0"]
impl crate::Resettable for USB_BGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
