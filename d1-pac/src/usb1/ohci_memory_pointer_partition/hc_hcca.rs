#[doc = "Register `hc_hcca` reader"]
pub type R = crate::R<HC_HCCA_SPEC>;
#[doc = "Register `hc_hcca` writer"]
pub type W = crate::W<HC_HCCA_SPEC>;
#[doc = "Field `hcca_7_0` reader - The alignment restriction in HcHCCA register is evaluated by examining the number of zeros in the lower order bits. The minimum alignment is 256 bytes, therefore, bits 0 through 7 must always return 0 when read."]
pub type HCCA_7_0_R = crate::FieldReader;
#[doc = "Field `hcca_31_8` reader - This is the base address of the Host Controller Communication Area. This area is used to hold the control structures and the Interrupt table that are accessed by both the Host Controller and the Host Controller Driver."]
pub type HCCA_31_8_R = crate::FieldReader<u32>;
#[doc = "Field `hcca_31_8` writer - This is the base address of the Host Controller Communication Area. This area is used to hold the control structures and the Interrupt table that are accessed by both the Host Controller and the Host Controller Driver."]
pub type HCCA_31_8_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - The alignment restriction in HcHCCA register is evaluated by examining the number of zeros in the lower order bits. The minimum alignment is 256 bytes, therefore, bits 0 through 7 must always return 0 when read."]
    #[inline(always)]
    pub fn hcca_7_0(&self) -> HCCA_7_0_R {
        HCCA_7_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - This is the base address of the Host Controller Communication Area. This area is used to hold the control structures and the Interrupt table that are accessed by both the Host Controller and the Host Controller Driver."]
    #[inline(always)]
    pub fn hcca_31_8(&self) -> HCCA_31_8_R {
        HCCA_31_8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - This is the base address of the Host Controller Communication Area. This area is used to hold the control structures and the Interrupt table that are accessed by both the Host Controller and the Host Controller Driver."]
    #[inline(always)]
    #[must_use]
    pub fn hcca_31_8(&mut self) -> HCCA_31_8_W<HC_HCCA_SPEC> {
        HCCA_31_8_W::new(self, 8)
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
#[doc = "OHCI HCCA Base\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_hcca::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_hcca::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC_HCCA_SPEC;
impl crate::RegisterSpec for HC_HCCA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc_hcca::R`](R) reader structure"]
impl crate::Readable for HC_HCCA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc_hcca::W`](W) writer structure"]
impl crate::Writable for HC_HCCA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_hcca to value 0"]
impl crate::Resettable for HC_HCCA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
