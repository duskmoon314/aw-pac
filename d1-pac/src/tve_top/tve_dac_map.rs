#[doc = "Register `tve_dac_map` reader"]
pub type R = crate::R<TVE_DAC_MAP_SPEC>;
#[doc = "Register `tve_dac_map` writer"]
pub type W = crate::W<TVE_DAC_MAP_SPEC>;
#[doc = "Field `dac_sel` reader - "]
pub type DAC_SEL_R = crate::FieldReader<DAC_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC_SEL_A {
    #[doc = "1: TVE0"]
    TVE0 = 1,
}
impl From<DAC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAC_SEL_A {
    type Ux = u8;
}
impl DAC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DAC_SEL_A> {
        match self.bits {
            1 => Some(DAC_SEL_A::TVE0),
            _ => None,
        }
    }
    #[doc = "TVE0"]
    #[inline(always)]
    pub fn is_tve0(&self) -> bool {
        *self == DAC_SEL_A::TVE0
    }
}
#[doc = "Field `dac_sel` writer - "]
pub type DAC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DAC_SEL_A>;
impl<'a, REG> DAC_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TVE0"]
    #[inline(always)]
    pub fn tve0(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_SEL_A::TVE0)
    }
}
#[doc = "Field `dac_map` reader - "]
pub type DAC_MAP_R = crate::FieldReader<DAC_MAP_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC_MAP_A {
    #[doc = "0: OUT0"]
    OUT0 = 0,
}
impl From<DAC_MAP_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC_MAP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAC_MAP_A {
    type Ux = u8;
}
impl DAC_MAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DAC_MAP_A> {
        match self.bits {
            0 => Some(DAC_MAP_A::OUT0),
            _ => None,
        }
    }
    #[doc = "OUT0"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == DAC_MAP_A::OUT0
    }
}
#[doc = "Field `dac_map` writer - "]
pub type DAC_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DAC_MAP_A>;
impl<'a, REG> DAC_MAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OUT0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_MAP_A::OUT0)
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dac_sel(&self) -> DAC_SEL_R {
        DAC_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dac_map(&self) -> DAC_MAP_R {
        DAC_MAP_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn dac_sel(&mut self) -> DAC_SEL_W<TVE_DAC_MAP_SPEC> {
        DAC_SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn dac_map(&mut self) -> DAC_MAP_W<TVE_DAC_MAP_SPEC> {
        DAC_MAP_W::new(self, 4)
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
#[doc = "TV Encoder DAC MAP Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_dac_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_dac_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_DAC_MAP_SPEC;
impl crate::RegisterSpec for TVE_DAC_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_dac_map::R`](R) reader structure"]
impl crate::Readable for TVE_DAC_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_dac_map::W`](W) writer structure"]
impl crate::Writable for TVE_DAC_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_dac_map to value 0"]
impl crate::Resettable for TVE_DAC_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
