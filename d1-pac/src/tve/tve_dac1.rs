#[doc = "Register `tve_dac1` reader"]
pub type R = crate::R<TVE_DAC1_SPEC>;
#[doc = "Register `tve_dac1` writer"]
pub type W = crate::W<TVE_DAC1_SPEC>;
#[doc = "Field `dac0_src_sel` reader - "]
pub type DAC0_SRC_SEL_R = crate::FieldReader<DAC0_SRC_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC0_SRC_SEL_A {
    #[doc = "0: Composite"]
    C_OMPOSITE = 0,
}
impl From<DAC0_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC0_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAC0_SRC_SEL_A {
    type Ux = u8;
}
impl DAC0_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DAC0_SRC_SEL_A> {
        match self.bits {
            0 => Some(DAC0_SRC_SEL_A::C_OMPOSITE),
            _ => None,
        }
    }
    #[doc = "Composite"]
    #[inline(always)]
    pub fn is_c_omposite(&self) -> bool {
        *self == DAC0_SRC_SEL_A::C_OMPOSITE
    }
}
#[doc = "Field `dac0_src_sel` writer - "]
pub type DAC0_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DAC0_SRC_SEL_A>;
impl<'a, REG> DAC0_SRC_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Composite"]
    #[inline(always)]
    pub fn c_omposite(self) -> &'a mut crate::W<REG> {
        self.variant(DAC0_SRC_SEL_A::C_OMPOSITE)
    }
}
impl R {
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dac0_src_sel(&self) -> DAC0_SRC_SEL_R {
        DAC0_SRC_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn dac0_src_sel(&mut self) -> DAC0_SRC_SEL_W<TVE_DAC1_SPEC> {
        DAC0_SRC_SEL_W::new(self, 4)
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
#[doc = "TV Encoder DAC Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_dac1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_dac1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_DAC1_SPEC;
impl crate::RegisterSpec for TVE_DAC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_dac1::R`](R) reader structure"]
impl crate::Readable for TVE_DAC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_dac1::W`](W) writer structure"]
impl crate::Writable for TVE_DAC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_dac1 to value 0"]
impl crate::Resettable for TVE_DAC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
