#[doc = "Register `periodiclistbase` reader"]
pub type R = crate::R<PERIODICLISTBASE_SPEC>;
#[doc = "Register `periodiclistbase` writer"]
pub type W = crate::W<PERIODICLISTBASE_SPEC>;
#[doc = "Field `base_address` reader - Base Address\n\nThese bits correspond to memory address signals \\[31:12\\], respectively.\n\nThis register contains the beginning address of the Periodic Frame List in the system memory.\n\nSystem software loads this register prior to starting the schedule execution by the Host Controller. The memory structure referenced by this physical memory pointer is assumed to be 4 Kbyte aligned. The contents of this register are combined with the Frame Index Register (FRINDEX) to enable the Host Controller to step through the Periodic Frame List in sequence."]
pub type BASE_ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `base_address` writer - Base Address\n\nThese bits correspond to memory address signals \\[31:12\\], respectively.\n\nThis register contains the beginning address of the Periodic Frame List in the system memory.\n\nSystem software loads this register prior to starting the schedule execution by the Host Controller. The memory structure referenced by this physical memory pointer is assumed to be 4 Kbyte aligned. The contents of this register are combined with the Frame Index Register (FRINDEX) to enable the Host Controller to step through the Periodic Frame List in sequence."]
pub type BASE_ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
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
    pub fn base_address(&mut self) -> BASE_ADDRESS_W<PERIODICLISTBASE_SPEC> {
        BASE_ADDRESS_W::new(self, 12)
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
#[doc = "EHCI Periodic Frame List Base Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periodiclistbase::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`periodiclistbase::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIODICLISTBASE_SPEC;
impl crate::RegisterSpec for PERIODICLISTBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periodiclistbase::R`](R) reader structure"]
impl crate::Readable for PERIODICLISTBASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`periodiclistbase::W`](W) writer structure"]
impl crate::Writable for PERIODICLISTBASE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets periodiclistbase to value 0"]
impl crate::Resettable for PERIODICLISTBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
