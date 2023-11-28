#[doc = "Register `pc_eint_cfg0` reader"]
pub type R = crate::R<PC_EINT_CFG0_SPEC>;
#[doc = "Register `pc_eint_cfg0` writer"]
pub type W = crate::W<PC_EINT_CFG0_SPEC>;
#[doc = "Field `eint_cfg[0-7]` reader - External INT Mode"]
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
#[doc = "Field `eint_cfg[0-7]` writer - External INT Mode"]
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
    #[doc = "External INT Mode\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `eint0_cfg` field"]
    #[inline(always)]
    pub fn eint_cfg(&self, n: u8) -> EINT_CFG_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        EINT_CFG_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - External INT Mode"]
    #[inline(always)]
    pub fn eint0_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External INT Mode"]
    #[inline(always)]
    pub fn eint1_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External INT Mode"]
    #[inline(always)]
    pub fn eint2_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External INT Mode"]
    #[inline(always)]
    pub fn eint3_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - External INT Mode"]
    #[inline(always)]
    pub fn eint4_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - External INT Mode"]
    #[inline(always)]
    pub fn eint5_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - External INT Mode"]
    #[inline(always)]
    pub fn eint6_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - External INT Mode"]
    #[inline(always)]
    pub fn eint7_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "External INT Mode\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `eint0_cfg` field"]
    #[inline(always)]
    #[must_use]
    pub fn eint_cfg(&mut self, n: u8) -> EINT_CFG_W<PC_EINT_CFG0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        EINT_CFG_W::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - External INT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eint0_cfg(&mut self) -> EINT_CFG_W<PC_EINT_CFG0_SPEC> {
        EINT_CFG_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - External INT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eint1_cfg(&mut self) -> EINT_CFG_W<PC_EINT_CFG0_SPEC> {
        EINT_CFG_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - External INT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eint2_cfg(&mut self) -> EINT_CFG_W<PC_EINT_CFG0_SPEC> {
        EINT_CFG_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - External INT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eint3_cfg(&mut self) -> EINT_CFG_W<PC_EINT_CFG0_SPEC> {
        EINT_CFG_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - External INT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eint4_cfg(&mut self) -> EINT_CFG_W<PC_EINT_CFG0_SPEC> {
        EINT_CFG_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - External INT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eint5_cfg(&mut self) -> EINT_CFG_W<PC_EINT_CFG0_SPEC> {
        EINT_CFG_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - External INT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eint6_cfg(&mut self) -> EINT_CFG_W<PC_EINT_CFG0_SPEC> {
        EINT_CFG_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - External INT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eint7_cfg(&mut self) -> EINT_CFG_W<PC_EINT_CFG0_SPEC> {
        EINT_CFG_W::new(self, 28)
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
#[doc = "PC External Interrupt Configure Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc_eint_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc_eint_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC_EINT_CFG0_SPEC;
impl crate::RegisterSpec for PC_EINT_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc_eint_cfg0::R`](R) reader structure"]
impl crate::Readable for PC_EINT_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pc_eint_cfg0::W`](W) writer structure"]
impl crate::Writable for PC_EINT_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pc_eint_cfg0 to value 0"]
impl crate::Resettable for PC_EINT_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
