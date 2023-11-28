#[doc = "Register `tv_src_ctl` reader"]
pub type R = crate::R<TV_SRC_CTL_SPEC>;
#[doc = "Register `tv_src_ctl` writer"]
pub type W = crate::W<TV_SRC_CTL_SPEC>;
#[doc = "Field `tv_src_sel` reader - TV Source Select"]
pub type TV_SRC_SEL_R = crate::FieldReader<TV_SRC_SEL_A>;
#[doc = "TV Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TV_SRC_SEL_A {
    #[doc = "0: DE"]
    DE = 0,
    #[doc = "1: Color Check"]
    COLOR = 1,
    #[doc = "2: Grayscale Check"]
    GRAYSCALE = 2,
    #[doc = "3: Black by White Check"]
    BLACK_BY_WHITE = 3,
    #[doc = "7: Gridding Check"]
    GRIDDING = 7,
}
impl From<TV_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TV_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TV_SRC_SEL_A {
    type Ux = u8;
}
impl TV_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TV_SRC_SEL_A> {
        match self.bits {
            0 => Some(TV_SRC_SEL_A::DE),
            1 => Some(TV_SRC_SEL_A::COLOR),
            2 => Some(TV_SRC_SEL_A::GRAYSCALE),
            3 => Some(TV_SRC_SEL_A::BLACK_BY_WHITE),
            7 => Some(TV_SRC_SEL_A::GRIDDING),
            _ => None,
        }
    }
    #[doc = "DE"]
    #[inline(always)]
    pub fn is_de(&self) -> bool {
        *self == TV_SRC_SEL_A::DE
    }
    #[doc = "Color Check"]
    #[inline(always)]
    pub fn is_color(&self) -> bool {
        *self == TV_SRC_SEL_A::COLOR
    }
    #[doc = "Grayscale Check"]
    #[inline(always)]
    pub fn is_grayscale(&self) -> bool {
        *self == TV_SRC_SEL_A::GRAYSCALE
    }
    #[doc = "Black by White Check"]
    #[inline(always)]
    pub fn is_black_by_white(&self) -> bool {
        *self == TV_SRC_SEL_A::BLACK_BY_WHITE
    }
    #[doc = "Gridding Check"]
    #[inline(always)]
    pub fn is_gridding(&self) -> bool {
        *self == TV_SRC_SEL_A::GRIDDING
    }
}
#[doc = "Field `tv_src_sel` writer - TV Source Select"]
pub type TV_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TV_SRC_SEL_A>;
impl<'a, REG> TV_SRC_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DE"]
    #[inline(always)]
    pub fn de(self) -> &'a mut crate::W<REG> {
        self.variant(TV_SRC_SEL_A::DE)
    }
    #[doc = "Color Check"]
    #[inline(always)]
    pub fn color(self) -> &'a mut crate::W<REG> {
        self.variant(TV_SRC_SEL_A::COLOR)
    }
    #[doc = "Grayscale Check"]
    #[inline(always)]
    pub fn grayscale(self) -> &'a mut crate::W<REG> {
        self.variant(TV_SRC_SEL_A::GRAYSCALE)
    }
    #[doc = "Black by White Check"]
    #[inline(always)]
    pub fn black_by_white(self) -> &'a mut crate::W<REG> {
        self.variant(TV_SRC_SEL_A::BLACK_BY_WHITE)
    }
    #[doc = "Gridding Check"]
    #[inline(always)]
    pub fn gridding(self) -> &'a mut crate::W<REG> {
        self.variant(TV_SRC_SEL_A::GRIDDING)
    }
}
impl R {
    #[doc = "Bits 0:2 - TV Source Select"]
    #[inline(always)]
    pub fn tv_src_sel(&self) -> TV_SRC_SEL_R {
        TV_SRC_SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - TV Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn tv_src_sel(&mut self) -> TV_SRC_SEL_W<TV_SRC_CTL_SPEC> {
        TV_SRC_SEL_W::new(self, 0)
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
#[doc = "TV Source Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_src_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_src_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_SRC_CTL_SPEC;
impl crate::RegisterSpec for TV_SRC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_src_ctl::R`](R) reader structure"]
impl crate::Readable for TV_SRC_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_src_ctl::W`](W) writer structure"]
impl crate::Writable for TV_SRC_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_src_ctl to value 0"]
impl crate::Resettable for TV_SRC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
