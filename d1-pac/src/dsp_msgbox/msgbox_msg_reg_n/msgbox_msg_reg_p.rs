#[doc = "Register `MSGBOX_MSG_REG_P%s` reader"]
pub struct R(crate::R<MSGBOX_MSG_REG_P_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSGBOX_MSG_REG_P_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSGBOX_MSG_REG_P_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSGBOX_MSG_REG_P_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSGBOX_MSG_REG_P%s` writer"]
pub struct W(crate::W<MSGBOX_MSG_REG_P_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSGBOX_MSG_REG_P_SPEC>;
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
impl From<crate::W<MSGBOX_MSG_REG_P_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSGBOX_MSG_REG_P_SPEC>) -> Self {
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
#[doc = "MSGBOX Message Queue Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msgbox_msg_reg_p](index.html) module"]
pub struct MSGBOX_MSG_REG_P_SPEC;
impl crate::RegisterSpec for MSGBOX_MSG_REG_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msgbox_msg_reg_p::R](R) reader structure"]
impl crate::Readable for MSGBOX_MSG_REG_P_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msgbox_msg_reg_p::W](W) writer structure"]
impl crate::Writable for MSGBOX_MSG_REG_P_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSGBOX_MSG_REG_P%s to value 0"]
impl crate::Resettable for MSGBOX_MSG_REG_P_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
