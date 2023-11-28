#[doc = "Register `cir_idc_h` reader"]
pub type R = crate::R<CIR_IDC_H_SPEC>;
#[doc = "Register `cir_idc_h` writer"]
pub type W = crate::W<CIR_IDC_H_SPEC>;
#[doc = "Field `idc_h` reader - Idle Duration Counter Threshold (High 4 bits)\n\nIdle Duration = 128*IDC*Ts (IDC = 0-4095)\n\nIt is used in cyclical transmission mode. When all the data in FIFO is transmitted, the signals can be transmitted after a specific time."]
pub type IDC_H_R = crate::FieldReader;
#[doc = "Field `idc_h` writer - Idle Duration Counter Threshold (High 4 bits)\n\nIdle Duration = 128*IDC*Ts (IDC = 0-4095)\n\nIt is used in cyclical transmission mode. When all the data in FIFO is transmitted, the signals can be transmitted after a specific time."]
pub type IDC_H_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Idle Duration Counter Threshold (High 4 bits)\n\nIdle Duration = 128*IDC*Ts (IDC = 0-4095)\n\nIt is used in cyclical transmission mode. When all the data in FIFO is transmitted, the signals can be transmitted after a specific time."]
    #[inline(always)]
    pub fn idc_h(&self) -> IDC_H_R {
        IDC_H_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Idle Duration Counter Threshold (High 4 bits)\n\nIdle Duration = 128*IDC*Ts (IDC = 0-4095)\n\nIt is used in cyclical transmission mode. When all the data in FIFO is transmitted, the signals can be transmitted after a specific time."]
    #[inline(always)]
    #[must_use]
    pub fn idc_h(&mut self) -> IDC_H_W<CIR_IDC_H_SPEC> {
        IDC_H_W::new(self, 0)
    }
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
#[doc = "CIR Transmit Idle Duration Threshold High Bit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_idc_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_idc_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIR_IDC_H_SPEC;
impl crate::RegisterSpec for CIR_IDC_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir_idc_h::R`](R) reader structure"]
impl crate::Readable for CIR_IDC_H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cir_idc_h::W`](W) writer structure"]
impl crate::Writable for CIR_IDC_H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_idc_h to value 0"]
impl crate::Resettable for CIR_IDC_H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
