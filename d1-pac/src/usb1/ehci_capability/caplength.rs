#[doc = "Register `caplength` reader"]
pub type R = crate::R<CAPLENGTH_SPEC>;
#[doc = "Field `caplength` reader - The value in these bits indicates an offset to add to register base to find the beginning of the Operational Register Space."]
pub type CAPLENGTH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The value in these bits indicates an offset to add to register base to find the beginning of the Operational Register Space."]
    #[inline(always)]
    pub fn caplength(&self) -> CAPLENGTH_R {
        CAPLENGTH_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "EHCI Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caplength::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPLENGTH_SPEC;
impl crate::RegisterSpec for CAPLENGTH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`caplength::R`](R) reader structure"]
impl crate::Readable for CAPLENGTH_SPEC {}
#[doc = "`reset()` method sets caplength to value 0"]
impl crate::Resettable for CAPLENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
