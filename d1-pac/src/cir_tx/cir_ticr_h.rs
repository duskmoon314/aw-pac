#[doc = "Register `cir_ticr_h` reader"]
pub struct R(crate::R<CIR_TICR_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_TICR_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_TICR_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_TICR_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cir_ticr_h` writer"]
pub struct W(crate::W<CIR_TICR_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_TICR_H_SPEC>;
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
impl From<crate::W<CIR_TICR_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_TICR_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tic_h` reader - Transmit Idle Counter_H (High 8 bits)\n\nIt is used to count the idle duration of CIR transmitter by software.\n\nCount in 128*Ts (Sample Duration, 1/Fs) when the transmitter is idle, and it should be reset when the transmitter is active.\n\nWhen this counter reaches the maximum value (0xFFFF), it will stop automatically, and should not be cleared to zero."]
pub type TIC_H_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Idle Counter_H (High 8 bits)\n\nIt is used to count the idle duration of CIR transmitter by software.\n\nCount in 128*Ts (Sample Duration, 1/Fs) when the transmitter is idle, and it should be reset when the transmitter is active.\n\nWhen this counter reaches the maximum value (0xFFFF), it will stop automatically, and should not be cleared to zero."]
    #[inline(always)]
    pub fn tic_h(&self) -> TIC_H_R {
        TIC_H_R::new((self.bits & 0xff) as u8)
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
#[doc = "CIR Transmit Idle Counter High Bit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_ticr_h](index.html) module"]
pub struct CIR_TICR_H_SPEC;
impl crate::RegisterSpec for CIR_TICR_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_ticr_h::R](R) reader structure"]
impl crate::Readable for CIR_TICR_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_ticr_h::W](W) writer structure"]
impl crate::Writable for CIR_TICR_H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_ticr_h to value 0"]
impl crate::Resettable for CIR_TICR_H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
