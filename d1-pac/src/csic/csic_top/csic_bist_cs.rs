#[doc = "Register `csic_bist_cs` reader"]
pub type R = crate::R<CSIC_BIST_CS_SPEC>;
#[doc = "Register `csic_bist_cs` writer"]
pub type W = crate::W<CSIC_BIST_CS_SPEC>;
#[doc = "Field `bist_cs` reader - "]
pub type BIST_CS_R = crate::FieldReader<BIST_CS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BIST_CS_A {
    #[doc = "0: Set when BK0 memory bist"]
    SET_BK0 = 0,
    #[doc = "1: Set when BK1 memory bist"]
    SET_BK1 = 1,
}
impl From<BIST_CS_A> for u8 {
    #[inline(always)]
    fn from(variant: BIST_CS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BIST_CS_A {
    type Ux = u8;
}
impl BIST_CS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BIST_CS_A> {
        match self.bits {
            0 => Some(BIST_CS_A::SET_BK0),
            1 => Some(BIST_CS_A::SET_BK1),
            _ => None,
        }
    }
    #[doc = "Set when BK0 memory bist"]
    #[inline(always)]
    pub fn is_set_bk0(&self) -> bool {
        *self == BIST_CS_A::SET_BK0
    }
    #[doc = "Set when BK1 memory bist"]
    #[inline(always)]
    pub fn is_set_bk1(&self) -> bool {
        *self == BIST_CS_A::SET_BK1
    }
}
#[doc = "Field `bist_cs` writer - "]
pub type BIST_CS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BIST_CS_A>;
impl<'a, REG> BIST_CS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set when BK0 memory bist"]
    #[inline(always)]
    pub fn set_bk0(self) -> &'a mut crate::W<REG> {
        self.variant(BIST_CS_A::SET_BK0)
    }
    #[doc = "Set when BK1 memory bist"]
    #[inline(always)]
    pub fn set_bk1(self) -> &'a mut crate::W<REG> {
        self.variant(BIST_CS_A::SET_BK1)
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn bist_cs(&self) -> BIST_CS_R {
        BIST_CS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn bist_cs(&mut self) -> BIST_CS_W<CSIC_BIST_CS_SPEC> {
        BIST_CS_W::new(self, 0)
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
#[doc = "CSIC BIST CS Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_bist_cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_bist_cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_BIST_CS_SPEC;
impl crate::RegisterSpec for CSIC_BIST_CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_bist_cs::R`](R) reader structure"]
impl crate::Readable for CSIC_BIST_CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_bist_cs::W`](W) writer structure"]
impl crate::Writable for CSIC_BIST_CS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_bist_cs to value 0"]
impl crate::Resettable for CSIC_BIST_CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
