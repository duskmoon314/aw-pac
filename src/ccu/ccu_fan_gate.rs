#[doc = "Register `CCU_FAN_GATE` reader"]
pub struct R(crate::R<CCU_FAN_GATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCU_FAN_GATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCU_FAN_GATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCU_FAN_GATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCU_FAN_GATE` writer"]
pub struct W(crate::W<CCU_FAN_GATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCU_FAN_GATE_SPEC>;
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
impl From<crate::W<CCU_FAN_GATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCU_FAN_GATE_SPEC>) -> Self {
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
#[doc = "CCU FANOUT CLOCK GATE Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccu_fan_gate](index.html) module"]
pub struct CCU_FAN_GATE_SPEC;
impl crate::RegisterSpec for CCU_FAN_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccu_fan_gate::R](R) reader structure"]
impl crate::Readable for CCU_FAN_GATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccu_fan_gate::W](W) writer structure"]
impl crate::Writable for CCU_FAN_GATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCU_FAN_GATE to value 0"]
impl crate::Resettable for CCU_FAN_GATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
