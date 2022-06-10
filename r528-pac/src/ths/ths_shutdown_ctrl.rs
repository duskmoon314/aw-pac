#[doc = "Register `THS_SHUTDOWN_CTRL` reader"]
pub struct R(crate::R<THS_SHUTDOWN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THS_SHUTDOWN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THS_SHUTDOWN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THS_SHUTDOWN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THS_SHUTDOWN_CTRL` writer"]
pub struct W(crate::W<THS_SHUTDOWN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THS_SHUTDOWN_CTRL_SPEC>;
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
impl From<crate::W<THS_SHUTDOWN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THS_SHUTDOWN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHUT_T_HOT` reader - Thermal sensor shutdown threshold for hot temperature"]
pub type SHUT_T_HOT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SHUT_T_HOT` writer - Thermal sensor shutdown threshold for hot temperature"]
pub type SHUT_T_HOT_W<'a> = crate::FieldWriter<'a, u32, THS_SHUTDOWN_CTRL_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - Thermal sensor shutdown threshold for hot temperature"]
    #[inline(always)]
    pub fn shut_t_hot(&self) -> SHUT_T_HOT_R {
        SHUT_T_HOT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Thermal sensor shutdown threshold for hot temperature"]
    #[inline(always)]
    pub fn shut_t_hot(&mut self) -> SHUT_T_HOT_W {
        SHUT_T_HOT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "THS Shutdown Threshold Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ths_shutdown_ctrl](index.html) module"]
pub struct THS_SHUTDOWN_CTRL_SPEC;
impl crate::RegisterSpec for THS_SHUTDOWN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ths_shutdown_ctrl::R](R) reader structure"]
impl crate::Readable for THS_SHUTDOWN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ths_shutdown_ctrl::W](W) writer structure"]
impl crate::Writable for THS_SHUTDOWN_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THS_SHUTDOWN_CTRL to value 0x04e9"]
impl crate::Resettable for THS_SHUTDOWN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04e9
    }
}
