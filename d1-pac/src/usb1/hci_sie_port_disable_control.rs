#[doc = "Register `HCI_SIE_PORT_DISABLE_CONTROL` reader"]
pub struct R(crate::R<HCI_SIE_PORT_DISABLE_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCI_SIE_PORT_DISABLE_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCI_SIE_PORT_DISABLE_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCI_SIE_PORT_DISABLE_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCI_SIE_PORT_DISABLE_CONTROL` writer"]
pub struct W(crate::W<HCI_SIE_PORT_DISABLE_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCI_SIE_PORT_DISABLE_CONTROL_SPEC>;
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
impl From<crate::W<HCI_SIE_PORT_DISABLE_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCI_SIE_PORT_DISABLE_CONTROL_SPEC>) -> Self {
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
#[doc = "HCI SIE Port Disable Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hci_sie_port_disable_control](index.html) module"]
pub struct HCI_SIE_PORT_DISABLE_CONTROL_SPEC;
impl crate::RegisterSpec for HCI_SIE_PORT_DISABLE_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hci_sie_port_disable_control::R](R) reader structure"]
impl crate::Readable for HCI_SIE_PORT_DISABLE_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hci_sie_port_disable_control::W](W) writer structure"]
impl crate::Writable for HCI_SIE_PORT_DISABLE_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCI_SIE_PORT_DISABLE_CONTROL to value 0"]
impl crate::Resettable for HCI_SIE_PORT_DISABLE_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
