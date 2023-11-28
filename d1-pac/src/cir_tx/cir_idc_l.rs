#[doc = "Register `cir_idc_l` reader"]
pub type R = crate::R<CIR_IDC_L_SPEC>;
#[doc = "Register `cir_idc_l` writer"]
pub type W = crate::W<CIR_IDC_L_SPEC>;
#[doc = "Field `idc_l` reader - Idle Duration Counter Threshold (Low 8 bits)\n\nIdle Duration = 128*IDC*Ts (IDC = 0-4095)\n\nIt is used in cyclical transmission mode. When all the data in FIFO is transmitted, the signals can be transmitted after a specific time."]
pub type IDC_L_R = crate::FieldReader;
#[doc = "Field `idc_l` writer - Idle Duration Counter Threshold (Low 8 bits)\n\nIdle Duration = 128*IDC*Ts (IDC = 0-4095)\n\nIt is used in cyclical transmission mode. When all the data in FIFO is transmitted, the signals can be transmitted after a specific time."]
pub type IDC_L_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Idle Duration Counter Threshold (Low 8 bits)\n\nIdle Duration = 128*IDC*Ts (IDC = 0-4095)\n\nIt is used in cyclical transmission mode. When all the data in FIFO is transmitted, the signals can be transmitted after a specific time."]
    #[inline(always)]
    pub fn idc_l(&self) -> IDC_L_R {
        IDC_L_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Idle Duration Counter Threshold (Low 8 bits)\n\nIdle Duration = 128*IDC*Ts (IDC = 0-4095)\n\nIt is used in cyclical transmission mode. When all the data in FIFO is transmitted, the signals can be transmitted after a specific time."]
    #[inline(always)]
    #[must_use]
    pub fn idc_l(&mut self) -> IDC_L_W<CIR_IDC_L_SPEC> {
        IDC_L_W::new(self, 0)
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
#[doc = "CIR Transmit Idle Duration Threshold Low Bit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_idc_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_idc_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIR_IDC_L_SPEC;
impl crate::RegisterSpec for CIR_IDC_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir_idc_l::R`](R) reader structure"]
impl crate::Readable for CIR_IDC_L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cir_idc_l::W`](W) writer structure"]
impl crate::Writable for CIR_IDC_L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_idc_l to value 0"]
impl crate::Resettable for CIR_IDC_L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
