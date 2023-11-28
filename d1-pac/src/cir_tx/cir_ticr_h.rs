#[doc = "Register `cir_ticr_h` reader"]
pub type R = crate::R<CIR_TICR_H_SPEC>;
#[doc = "Register `cir_ticr_h` writer"]
pub type W = crate::W<CIR_TICR_H_SPEC>;
#[doc = "Field `tic_h` reader - Transmit Idle Counter_H (High 8 bits)\n\nIt is used to count the idle duration of CIR transmitter by software.\n\nCount in 128*Ts (Sample Duration, 1/Fs) when the transmitter is idle, and it should be reset when the transmitter is active.\n\nWhen this counter reaches the maximum value (0xFFFF), it will stop automatically, and should not be cleared to zero."]
pub type TIC_H_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit Idle Counter_H (High 8 bits)\n\nIt is used to count the idle duration of CIR transmitter by software.\n\nCount in 128*Ts (Sample Duration, 1/Fs) when the transmitter is idle, and it should be reset when the transmitter is active.\n\nWhen this counter reaches the maximum value (0xFFFF), it will stop automatically, and should not be cleared to zero."]
    #[inline(always)]
    pub fn tic_h(&self) -> TIC_H_R {
        TIC_H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
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
#[doc = "CIR Transmit Idle Counter High Bit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_ticr_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_ticr_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIR_TICR_H_SPEC;
impl crate::RegisterSpec for CIR_TICR_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir_ticr_h::R`](R) reader structure"]
impl crate::Readable for CIR_TICR_H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cir_ticr_h::W`](W) writer structure"]
impl crate::Writable for CIR_TICR_H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_ticr_h to value 0"]
impl crate::Resettable for CIR_TICR_H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
