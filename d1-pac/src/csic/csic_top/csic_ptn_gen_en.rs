#[doc = "Register `csic_ptn_gen_en` reader"]
pub type R = crate::R<CSIC_PTN_GEN_EN_SPEC>;
#[doc = "Register `csic_ptn_gen_en` writer"]
pub type W = crate::W<CSIC_PTN_GEN_EN_SPEC>;
#[doc = "Field `ptn_gen_en` reader - Pattern Generation Enable"]
pub type PTN_GEN_EN_R = crate::BitReader;
#[doc = "Field `ptn_gen_en` writer - Pattern Generation Enable"]
pub type PTN_GEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ptn_start` reader - CSIC Pattern Generating Start\n\nSoftware write this bit to \"1\" to start pattern generating from DRAM. When finished, the hardware will clear this bit to \"0\" automatically. Generating cycles depends on PTN_CYCLE."]
pub type PTN_START_R = crate::BitReader<PTN_START_A>;
#[doc = "CSIC Pattern Generating Start\n\nSoftware write this bit to \"1\" to start pattern generating from DRAM. When finished, the hardware will clear this bit to \"0\" automatically. Generating cycles depends on PTN_CYCLE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTN_START_A {
    #[doc = "0: Finish"]
    F_INISH = 0,
    #[doc = "1: Start"]
    S_TART = 1,
}
impl From<PTN_START_A> for bool {
    #[inline(always)]
    fn from(variant: PTN_START_A) -> Self {
        variant as u8 != 0
    }
}
impl PTN_START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PTN_START_A {
        match self.bits {
            false => PTN_START_A::F_INISH,
            true => PTN_START_A::S_TART,
        }
    }
    #[doc = "Finish"]
    #[inline(always)]
    pub fn is_f_inish(&self) -> bool {
        *self == PTN_START_A::F_INISH
    }
    #[doc = "Start"]
    #[inline(always)]
    pub fn is_s_tart(&self) -> bool {
        *self == PTN_START_A::S_TART
    }
}
#[doc = "Field `ptn_start` writer - CSIC Pattern Generating Start\n\nSoftware write this bit to \"1\" to start pattern generating from DRAM. When finished, the hardware will clear this bit to \"0\" automatically. Generating cycles depends on PTN_CYCLE."]
pub type PTN_START_W<'a, REG> = crate::BitWriter<'a, REG, PTN_START_A>;
impl<'a, REG> PTN_START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Finish"]
    #[inline(always)]
    pub fn f_inish(self) -> &'a mut crate::W<REG> {
        self.variant(PTN_START_A::F_INISH)
    }
    #[doc = "Start"]
    #[inline(always)]
    pub fn s_tart(self) -> &'a mut crate::W<REG> {
        self.variant(PTN_START_A::S_TART)
    }
}
#[doc = "Field `ptn_cycle` reader - Pattern generating cycle counter.\n\nThe pattern in dram will be generated in cycles of PTN_CYCLE+1."]
pub type PTN_CYCLE_R = crate::FieldReader;
#[doc = "Field `ptn_cycle` writer - Pattern generating cycle counter.\n\nThe pattern in dram will be generated in cycles of PTN_CYCLE+1."]
pub type PTN_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Pattern Generation Enable"]
    #[inline(always)]
    pub fn ptn_gen_en(&self) -> PTN_GEN_EN_R {
        PTN_GEN_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CSIC Pattern Generating Start\n\nSoftware write this bit to \"1\" to start pattern generating from DRAM. When finished, the hardware will clear this bit to \"0\" automatically. Generating cycles depends on PTN_CYCLE."]
    #[inline(always)]
    pub fn ptn_start(&self) -> PTN_START_R {
        PTN_START_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Pattern generating cycle counter.\n\nThe pattern in dram will be generated in cycles of PTN_CYCLE+1."]
    #[inline(always)]
    pub fn ptn_cycle(&self) -> PTN_CYCLE_R {
        PTN_CYCLE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Pattern Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ptn_gen_en(&mut self) -> PTN_GEN_EN_W<CSIC_PTN_GEN_EN_SPEC> {
        PTN_GEN_EN_W::new(self, 0)
    }
    #[doc = "Bit 4 - CSIC Pattern Generating Start\n\nSoftware write this bit to \"1\" to start pattern generating from DRAM. When finished, the hardware will clear this bit to \"0\" automatically. Generating cycles depends on PTN_CYCLE."]
    #[inline(always)]
    #[must_use]
    pub fn ptn_start(&mut self) -> PTN_START_W<CSIC_PTN_GEN_EN_SPEC> {
        PTN_START_W::new(self, 4)
    }
    #[doc = "Bits 16:23 - Pattern generating cycle counter.\n\nThe pattern in dram will be generated in cycles of PTN_CYCLE+1."]
    #[inline(always)]
    #[must_use]
    pub fn ptn_cycle(&mut self) -> PTN_CYCLE_W<CSIC_PTN_GEN_EN_SPEC> {
        PTN_CYCLE_W::new(self, 16)
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
#[doc = "CSIC Pattern Generation Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_ptn_gen_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_ptn_gen_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_PTN_GEN_EN_SPEC;
impl crate::RegisterSpec for CSIC_PTN_GEN_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_ptn_gen_en::R`](R) reader structure"]
impl crate::Readable for CSIC_PTN_GEN_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_ptn_gen_en::W`](W) writer structure"]
impl crate::Writable for CSIC_PTN_GEN_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_ptn_gen_en to value 0"]
impl crate::Resettable for CSIC_PTN_GEN_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
