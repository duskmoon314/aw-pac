#[doc = "Register `tve_dac_status` reader"]
pub type R = crate::R<TVE_DAC_STATUS_SPEC>;
#[doc = "Register `tve_dac_status` writer"]
pub type W = crate::W<TVE_DAC_STATUS_SPEC>;
#[doc = "Field `dac_status` reader - "]
pub type DAC_STATUS_R = crate::FieldReader<DAC_STATUS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC_STATUS_A {
    #[doc = "0: Unconnected"]
    U_NCONNECTED = 0,
    #[doc = "1: Connected"]
    C_ONNECTED = 1,
    #[doc = "3: Short to ground"]
    S_HORT = 3,
}
impl From<DAC_STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC_STATUS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAC_STATUS_A {
    type Ux = u8;
}
impl DAC_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DAC_STATUS_A> {
        match self.bits {
            0 => Some(DAC_STATUS_A::U_NCONNECTED),
            1 => Some(DAC_STATUS_A::C_ONNECTED),
            3 => Some(DAC_STATUS_A::S_HORT),
            _ => None,
        }
    }
    #[doc = "Unconnected"]
    #[inline(always)]
    pub fn is_u_nconnected(&self) -> bool {
        *self == DAC_STATUS_A::U_NCONNECTED
    }
    #[doc = "Connected"]
    #[inline(always)]
    pub fn is_c_onnected(&self) -> bool {
        *self == DAC_STATUS_A::C_ONNECTED
    }
    #[doc = "Short to ground"]
    #[inline(always)]
    pub fn is_s_hort(&self) -> bool {
        *self == DAC_STATUS_A::S_HORT
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dac_status(&self) -> DAC_STATUS_R {
        DAC_STATUS_R::new((self.bits & 3) as u8)
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
#[doc = "TV Encoder DAC STAUTS Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_dac_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_dac_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_DAC_STATUS_SPEC;
impl crate::RegisterSpec for TVE_DAC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_dac_status::R`](R) reader structure"]
impl crate::Readable for TVE_DAC_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_dac_status::W`](W) writer structure"]
impl crate::Writable for TVE_DAC_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_dac_status to value 0"]
impl crate::Resettable for TVE_DAC_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
