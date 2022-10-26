#[doc = "Register `periodiclistbase` reader"]
pub struct R(crate::R<PERIODICLISTBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIODICLISTBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIODICLISTBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIODICLISTBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `periodiclistbase` writer"]
pub struct W(crate::W<PERIODICLISTBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIODICLISTBASE_SPEC>;
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
impl From<crate::W<PERIODICLISTBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIODICLISTBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `base_address` reader - Base Address\n\nThese bits correspond to memory address signals \\[31:12\\], respectively.\n\nThis register contains the beginning address of the Periodic Frame List in the system memory.\n\nSystem software loads this register prior to starting the schedule execution by the Host Controller. The memory structure referenced by this physical memory pointer is assumed to be 4 Kbyte aligned. The contents of this register are combined with the Frame Index Register (FRINDEX) to enable the Host Controller to step through the Periodic Frame List in sequence."]
pub type BASE_ADDRESS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `base_address` writer - Base Address\n\nThese bits correspond to memory address signals \\[31:12\\], respectively.\n\nThis register contains the beginning address of the Periodic Frame List in the system memory.\n\nSystem software loads this register prior to starting the schedule execution by the Host Controller. The memory structure referenced by this physical memory pointer is assumed to be 4 Kbyte aligned. The contents of this register are combined with the Frame Index Register (FRINDEX) to enable the Host Controller to step through the Periodic Frame List in sequence."]
pub type BASE_ADDRESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERIODICLISTBASE_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 12:31 - Base Address\n\nThese bits correspond to memory address signals \\[31:12\\], respectively.\n\nThis register contains the beginning address of the Periodic Frame List in the system memory.\n\nSystem software loads this register prior to starting the schedule execution by the Host Controller. The memory structure referenced by this physical memory pointer is assumed to be 4 Kbyte aligned. The contents of this register are combined with the Frame Index Register (FRINDEX) to enable the Host Controller to step through the Periodic Frame List in sequence."]
    #[inline(always)]
    pub fn base_address(&self) -> BASE_ADDRESS_R {
        BASE_ADDRESS_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - Base Address\n\nThese bits correspond to memory address signals \\[31:12\\], respectively.\n\nThis register contains the beginning address of the Periodic Frame List in the system memory.\n\nSystem software loads this register prior to starting the schedule execution by the Host Controller. The memory structure referenced by this physical memory pointer is assumed to be 4 Kbyte aligned. The contents of this register are combined with the Frame Index Register (FRINDEX) to enable the Host Controller to step through the Periodic Frame List in sequence."]
    #[inline(always)]
    #[must_use]
    pub fn base_address(&mut self) -> BASE_ADDRESS_W<12> {
        BASE_ADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EHCI Periodic Frame List Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periodiclistbase](index.html) module"]
pub struct PERIODICLISTBASE_SPEC;
impl crate::RegisterSpec for PERIODICLISTBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periodiclistbase::R](R) reader structure"]
impl crate::Readable for PERIODICLISTBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [periodiclistbase::W](W) writer structure"]
impl crate::Writable for PERIODICLISTBASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets periodiclistbase to value 0"]
impl crate::Resettable for PERIODICLISTBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
