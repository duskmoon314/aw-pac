#[doc = "Register `pe_eint_cfg2` reader"]
pub type R = crate::R<PE_EINT_CFG2_SPEC>;
#[doc = "Register `pe_eint_cfg2` writer"]
pub type W = crate::W<PE_EINT_CFG2_SPEC>;
#[doc = "Field `eint_cfg[16-17]` reader - External INT Mode"]
pub type EINT_CFG_R = crate::FieldReader<EINT_CFG_A>;
#[doc = "External INT Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EINT_CFG_A {
    #[doc = "0: `0`"]
    POSITIVE_EDGE = 0,
    #[doc = "1: `1`"]
    NEGATIVE_EDGE = 1,
    #[doc = "2: `10`"]
    HIGH_LEVEL = 2,
    #[doc = "3: `11`"]
    LOW_LEVEL = 3,
    #[doc = "4: `100`"]
    DOUBLE_EDGE = 4,
}
impl From<EINT_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: EINT_CFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EINT_CFG_A {
    type Ux = u8;
}
impl EINT_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EINT_CFG_A> {
        match self.bits {
            0 => Some(EINT_CFG_A::POSITIVE_EDGE),
            1 => Some(EINT_CFG_A::NEGATIVE_EDGE),
            2 => Some(EINT_CFG_A::HIGH_LEVEL),
            3 => Some(EINT_CFG_A::LOW_LEVEL),
            4 => Some(EINT_CFG_A::DOUBLE_EDGE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_positive_edge(&self) -> bool {
        *self == EINT_CFG_A::POSITIVE_EDGE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_negative_edge(&self) -> bool {
        *self == EINT_CFG_A::NEGATIVE_EDGE
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == EINT_CFG_A::HIGH_LEVEL
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == EINT_CFG_A::LOW_LEVEL
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_double_edge(&self) -> bool {
        *self == EINT_CFG_A::DOUBLE_EDGE
    }
}
#[doc = "Field `eint_cfg[16-17]` writer - External INT Mode"]
pub type EINT_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EINT_CFG_A>;
impl<'a, REG> EINT_CFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn positive_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EINT_CFG_A::POSITIVE_EDGE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn negative_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EINT_CFG_A::NEGATIVE_EDGE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut crate::W<REG> {
        self.variant(EINT_CFG_A::HIGH_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut crate::W<REG> {
        self.variant(EINT_CFG_A::LOW_LEVEL)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn double_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EINT_CFG_A::DOUBLE_EDGE)
    }
}
impl R {
    #[doc = "External INT Mode\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `eint16_cfg` field"]
    #[inline(always)]
    pub fn eint_cfg(&self, n: u8) -> EINT_CFG_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        EINT_CFG_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - External INT Mode"]
    #[inline(always)]
    pub fn eint16_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External INT Mode"]
    #[inline(always)]
    pub fn eint17_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "External INT Mode\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `eint16_cfg` field"]
    #[inline(always)]
    #[must_use]
    pub fn eint_cfg(&mut self, n: u8) -> EINT_CFG_W<PE_EINT_CFG2_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        EINT_CFG_W::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - External INT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eint16_cfg(&mut self) -> EINT_CFG_W<PE_EINT_CFG2_SPEC> {
        EINT_CFG_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - External INT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eint17_cfg(&mut self) -> EINT_CFG_W<PE_EINT_CFG2_SPEC> {
        EINT_CFG_W::new(self, 4)
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
#[doc = "PE External Interrupt Configure Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_eint_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_eint_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PE_EINT_CFG2_SPEC;
impl crate::RegisterSpec for PE_EINT_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_eint_cfg2::R`](R) reader structure"]
impl crate::Readable for PE_EINT_CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pe_eint_cfg2::W`](W) writer structure"]
impl crate::Writable for PE_EINT_CFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pe_eint_cfg2 to value 0"]
impl crate::Resettable for PE_EINT_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
