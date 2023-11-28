#[doc = "Register `ac_dac_drc_hopc` reader"]
pub type R = crate::R<AC_DAC_DRC_HOPC_SPEC>;
#[doc = "Register `ac_dac_drc_hopc` writer"]
pub type W = crate::W<AC_DAC_DRC_HOPC_SPEC>;
#[doc = "Field `dac_drc_hopc` reader - The output of the compressor, which is determined by the equation -OPC/6.0206. The format is 8.24 (The default value is -40 dB)"]
pub type DAC_DRC_HOPC_R = crate::FieldReader<u16>;
#[doc = "Field `dac_drc_hopc` writer - The output of the compressor, which is determined by the equation -OPC/6.0206. The format is 8.24 (The default value is -40 dB)"]
pub type DAC_DRC_HOPC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The output of the compressor, which is determined by the equation -OPC/6.0206. The format is 8.24 (The default value is -40 dB)"]
    #[inline(always)]
    pub fn dac_drc_hopc(&self) -> DAC_DRC_HOPC_R {
        DAC_DRC_HOPC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The output of the compressor, which is determined by the equation -OPC/6.0206. The format is 8.24 (The default value is -40 dB)"]
    #[inline(always)]
    #[must_use]
    pub fn dac_drc_hopc(&mut self) -> DAC_DRC_HOPC_W<AC_DAC_DRC_HOPC_SPEC> {
        DAC_DRC_HOPC_W::new(self, 0)
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
#[doc = "DAC DRC Compressor High Output at Compressor Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hopc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hopc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_DAC_DRC_HOPC_SPEC;
impl crate::RegisterSpec for AC_DAC_DRC_HOPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_dac_drc_hopc::R`](R) reader structure"]
impl crate::Readable for AC_DAC_DRC_HOPC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_dac_drc_hopc::W`](W) writer structure"]
impl crate::Writable for AC_DAC_DRC_HOPC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_drc_hopc to value 0xf95b"]
impl crate::Resettable for AC_DAC_DRC_HOPC_SPEC {
    const RESET_VALUE: Self::Ux = 0xf95b;
}
