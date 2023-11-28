#[doc = "Register `tve_dac_test` reader"]
pub type R = crate::R<TVE_DAC_TEST_SPEC>;
#[doc = "Register `tve_dac_test` writer"]
pub type W = crate::W<TVE_DAC_TEST_SPEC>;
#[doc = "Field `dac_test_enable` reader - "]
pub type DAC_TEST_ENABLE_R = crate::BitReader<DAC_TEST_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_TEST_ENABLE_A {
    #[doc = "1: Repeat DAC data from DAC sram"]
    R_EPEAT = 1,
}
impl From<DAC_TEST_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DAC_TEST_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC_TEST_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DAC_TEST_ENABLE_A> {
        match self.bits {
            true => Some(DAC_TEST_ENABLE_A::R_EPEAT),
            _ => None,
        }
    }
    #[doc = "Repeat DAC data from DAC sram"]
    #[inline(always)]
    pub fn is_r_epeat(&self) -> bool {
        *self == DAC_TEST_ENABLE_A::R_EPEAT
    }
}
#[doc = "Field `dac_test_enable` writer - "]
pub type DAC_TEST_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, DAC_TEST_ENABLE_A>;
impl<'a, REG> DAC_TEST_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Repeat DAC data from DAC sram"]
    #[inline(always)]
    pub fn r_epeat(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_TEST_ENABLE_A::R_EPEAT)
    }
}
#[doc = "Field `dac_test_sel` reader - "]
pub type DAC_TEST_SEL_R = crate::FieldReader<DAC_TEST_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC_TEST_SEL_A {
    #[doc = "0: DAC0"]
    DAC0 = 0,
}
impl From<DAC_TEST_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC_TEST_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAC_TEST_SEL_A {
    type Ux = u8;
}
impl DAC_TEST_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DAC_TEST_SEL_A> {
        match self.bits {
            0 => Some(DAC_TEST_SEL_A::DAC0),
            _ => None,
        }
    }
    #[doc = "DAC0"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == DAC_TEST_SEL_A::DAC0
    }
}
#[doc = "Field `dac_test_sel` writer - "]
pub type DAC_TEST_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DAC_TEST_SEL_A>;
impl<'a, REG> DAC_TEST_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DAC0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_TEST_SEL_A::DAC0)
    }
}
#[doc = "Field `dac_test_length` reader - DAC TEST DATA LENGTH"]
pub type DAC_TEST_LENGTH_R = crate::FieldReader<u16>;
#[doc = "Field `dac_test_length` writer - DAC TEST DATA LENGTH"]
pub type DAC_TEST_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dac_test_enable(&self) -> DAC_TEST_ENABLE_R {
        DAC_TEST_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dac_test_sel(&self) -> DAC_TEST_SEL_R {
        DAC_TEST_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:25 - DAC TEST DATA LENGTH"]
    #[inline(always)]
    pub fn dac_test_length(&self) -> DAC_TEST_LENGTH_R {
        DAC_TEST_LENGTH_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dac_test_enable(&mut self) -> DAC_TEST_ENABLE_W<TVE_DAC_TEST_SPEC> {
        DAC_TEST_ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn dac_test_sel(&mut self) -> DAC_TEST_SEL_W<TVE_DAC_TEST_SPEC> {
        DAC_TEST_SEL_W::new(self, 4)
    }
    #[doc = "Bits 16:25 - DAC TEST DATA LENGTH"]
    #[inline(always)]
    #[must_use]
    pub fn dac_test_length(&mut self) -> DAC_TEST_LENGTH_W<TVE_DAC_TEST_SPEC> {
        DAC_TEST_LENGTH_W::new(self, 16)
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
#[doc = "TV Encoder DAC TEST Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_dac_test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_dac_test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_DAC_TEST_SPEC;
impl crate::RegisterSpec for TVE_DAC_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_dac_test::R`](R) reader structure"]
impl crate::Readable for TVE_DAC_TEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_dac_test::W`](W) writer structure"]
impl crate::Writable for TVE_DAC_TEST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_dac_test to value 0"]
impl crate::Resettable for TVE_DAC_TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
