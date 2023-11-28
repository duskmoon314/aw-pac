#[doc = "Register `pe_pull1` reader"]
pub type R = crate::R<PE_PULL1_SPEC>;
#[doc = "Register `pe_pull1` writer"]
pub type W = crate::W<PE_PULL1_SPEC>;
#[doc = "Field `pe_pull[16-17]` reader - PE Pull_up/down Select"]
pub type PE_PULL_R = crate::FieldReader<PE_PULL_A>;
#[doc = "PE Pull_up/down Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PE_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PE_PULL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PE_PULL_A {
    type Ux = u8;
}
impl PE_PULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PE_PULL_A {
        match self.bits {
            0 => PE_PULL_A::PULL_DISABLE,
            1 => PE_PULL_A::PULL_UP,
            2 => PE_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        *self == PE_PULL_A::PULL_DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PE_PULL_A::PULL_UP
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PE_PULL_A::PULL_DOWN
    }
}
#[doc = "Field `pe_pull[16-17]` writer - PE Pull_up/down Select"]
pub type PE_PULL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PE_PULL_A>;
impl<'a, REG> PE_PULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PE_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PE_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PE_PULL_A::PULL_DOWN)
    }
}
impl R {
    #[doc = "PE Pull_up/down Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pe16_pull` field"]
    #[inline(always)]
    pub fn pe_pull(&self, n: u8) -> PE_PULL_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        PE_PULL_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PE Pull_up/down Select"]
    #[inline(always)]
    pub fn pe16_pull(&self) -> PE_PULL_R {
        PE_PULL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PE Pull_up/down Select"]
    #[inline(always)]
    pub fn pe17_pull(&self) -> PE_PULL_R {
        PE_PULL_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "PE Pull_up/down Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pe16_pull` field"]
    #[inline(always)]
    #[must_use]
    pub fn pe_pull(&mut self, n: u8) -> PE_PULL_W<PE_PULL1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        PE_PULL_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - PE Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe16_pull(&mut self) -> PE_PULL_W<PE_PULL1_SPEC> {
        PE_PULL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - PE Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe17_pull(&mut self) -> PE_PULL_W<PE_PULL1_SPEC> {
        PE_PULL_W::new(self, 2)
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
#[doc = "PE Pull Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_pull1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_pull1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PE_PULL1_SPEC;
impl crate::RegisterSpec for PE_PULL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_pull1::R`](R) reader structure"]
impl crate::Readable for PE_PULL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pe_pull1::W`](W) writer structure"]
impl crate::Writable for PE_PULL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pe_pull1 to value 0"]
impl crate::Resettable for PE_PULL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
