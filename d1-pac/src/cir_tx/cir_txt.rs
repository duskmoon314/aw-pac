#[doc = "Register `cir_txt` reader"]
pub type R = crate::R<CIR_TXT_SPEC>;
#[doc = "Register `cir_txt` writer"]
pub type W = crate::W<CIR_TXT_SPEC>;
#[doc = "Field `nctt` reader - Non-cyclical Pulse Transmit Threshold\n\nThe controller will trigger transmitting the data in the FIFO when the data byte number has reached the Transmit Threshold set in this field."]
pub type NCTT_R = crate::FieldReader;
#[doc = "Field `nctt` writer - Non-cyclical Pulse Transmit Threshold\n\nThe controller will trigger transmitting the data in the FIFO when the data byte number has reached the Transmit Threshold set in this field."]
pub type NCTT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Non-cyclical Pulse Transmit Threshold\n\nThe controller will trigger transmitting the data in the FIFO when the data byte number has reached the Transmit Threshold set in this field."]
    #[inline(always)]
    pub fn nctt(&self) -> NCTT_R {
        NCTT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Non-cyclical Pulse Transmit Threshold\n\nThe controller will trigger transmitting the data in the FIFO when the data byte number has reached the Transmit Threshold set in this field."]
    #[inline(always)]
    #[must_use]
    pub fn nctt(&mut self) -> NCTT_W<CIR_TXT_SPEC> {
        NCTT_W::new(self, 0)
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
#[doc = "CIR Transmit Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir_txt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir_txt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIR_TXT_SPEC;
impl crate::RegisterSpec for CIR_TXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir_txt::R`](R) reader structure"]
impl crate::Readable for CIR_TXT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cir_txt::W`](W) writer structure"]
impl crate::Writable for CIR_TXT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cir_txt to value 0"]
impl crate::Resettable for CIR_TXT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
