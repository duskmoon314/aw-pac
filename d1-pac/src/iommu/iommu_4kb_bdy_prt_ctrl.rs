#[doc = "Register `iommu_4kb_bdy_prt_ctrl` reader"]
pub type R = crate::R<IOMMU_4KB_BDY_PRT_CTRL_SPEC>;
#[doc = "Register `iommu_4kb_bdy_prt_ctrl` writer"]
pub type W = crate::W<IOMMU_4KB_BDY_PRT_CTRL_SPEC>;
#[doc = "Field `m_4kb_bdy_prt_ctrl[0-6]` reader - Master\\[i\\] 4 KB boundary protect control bit"]
pub type M_4KB_BDY_PRT_CTRL_R = crate::BitReader<M_4KB_BDY_PRT_CTRL_A>;
#[doc = "Master\\[i\\] 4 KB boundary protect control bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_4KB_BDY_PRT_CTRL_A {
    #[doc = "0: Disable 4 KB boundary protect"]
    DISABLE = 0,
    #[doc = "1: Enable 4 KB boundary protect"]
    ENABLE = 1,
}
impl From<M_4KB_BDY_PRT_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: M_4KB_BDY_PRT_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl M_4KB_BDY_PRT_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_4KB_BDY_PRT_CTRL_A {
        match self.bits {
            false => M_4KB_BDY_PRT_CTRL_A::DISABLE,
            true => M_4KB_BDY_PRT_CTRL_A::ENABLE,
        }
    }
    #[doc = "Disable 4 KB boundary protect"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == M_4KB_BDY_PRT_CTRL_A::DISABLE
    }
    #[doc = "Enable 4 KB boundary protect"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == M_4KB_BDY_PRT_CTRL_A::ENABLE
    }
}
#[doc = "Field `m_4kb_bdy_prt_ctrl[0-6]` writer - Master\\[i\\] 4 KB boundary protect control bit"]
pub type M_4KB_BDY_PRT_CTRL_W<'a, REG> = crate::BitWriter<'a, REG, M_4KB_BDY_PRT_CTRL_A>;
impl<'a, REG> M_4KB_BDY_PRT_CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable 4 KB boundary protect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(M_4KB_BDY_PRT_CTRL_A::DISABLE)
    }
    #[doc = "Enable 4 KB boundary protect"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(M_4KB_BDY_PRT_CTRL_A::ENABLE)
    }
}
impl R {
    #[doc = "Master\\[i\\] 4 KB boundary protect control bit\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `m0_4kb_bdy_prt_ctrl` field"]
    #[inline(always)]
    pub fn m_4kb_bdy_prt_ctrl(&self, n: u8) -> M_4KB_BDY_PRT_CTRL_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        M_4KB_BDY_PRT_CTRL_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Master\\[i\\] 4 KB boundary protect control bit"]
    #[inline(always)]
    pub fn m0_4kb_bdy_prt_ctrl(&self) -> M_4KB_BDY_PRT_CTRL_R {
        M_4KB_BDY_PRT_CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master\\[i\\] 4 KB boundary protect control bit"]
    #[inline(always)]
    pub fn m1_4kb_bdy_prt_ctrl(&self) -> M_4KB_BDY_PRT_CTRL_R {
        M_4KB_BDY_PRT_CTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master\\[i\\] 4 KB boundary protect control bit"]
    #[inline(always)]
    pub fn m2_4kb_bdy_prt_ctrl(&self) -> M_4KB_BDY_PRT_CTRL_R {
        M_4KB_BDY_PRT_CTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master\\[i\\] 4 KB boundary protect control bit"]
    #[inline(always)]
    pub fn m3_4kb_bdy_prt_ctrl(&self) -> M_4KB_BDY_PRT_CTRL_R {
        M_4KB_BDY_PRT_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master\\[i\\] 4 KB boundary protect control bit"]
    #[inline(always)]
    pub fn m4_4kb_bdy_prt_ctrl(&self) -> M_4KB_BDY_PRT_CTRL_R {
        M_4KB_BDY_PRT_CTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master\\[i\\] 4 KB boundary protect control bit"]
    #[inline(always)]
    pub fn m5_4kb_bdy_prt_ctrl(&self) -> M_4KB_BDY_PRT_CTRL_R {
        M_4KB_BDY_PRT_CTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master\\[i\\] 4 KB boundary protect control bit"]
    #[inline(always)]
    pub fn m6_4kb_bdy_prt_ctrl(&self) -> M_4KB_BDY_PRT_CTRL_R {
        M_4KB_BDY_PRT_CTRL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Master\\[i\\] 4 KB boundary protect control bit\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `m0_4kb_bdy_prt_ctrl` field"]
    #[inline(always)]
    #[must_use]
    pub fn m_4kb_bdy_prt_ctrl(
        &mut self,
        n: u8,
    ) -> M_4KB_BDY_PRT_CTRL_W<IOMMU_4KB_BDY_PRT_CTRL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        M_4KB_BDY_PRT_CTRL_W::new(self, n)
    }
    #[doc = "Bit 0 - Master\\[i\\] 4 KB boundary protect control bit"]
    #[inline(always)]
    #[must_use]
    pub fn m0_4kb_bdy_prt_ctrl(&mut self) -> M_4KB_BDY_PRT_CTRL_W<IOMMU_4KB_BDY_PRT_CTRL_SPEC> {
        M_4KB_BDY_PRT_CTRL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Master\\[i\\] 4 KB boundary protect control bit"]
    #[inline(always)]
    #[must_use]
    pub fn m1_4kb_bdy_prt_ctrl(&mut self) -> M_4KB_BDY_PRT_CTRL_W<IOMMU_4KB_BDY_PRT_CTRL_SPEC> {
        M_4KB_BDY_PRT_CTRL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Master\\[i\\] 4 KB boundary protect control bit"]
    #[inline(always)]
    #[must_use]
    pub fn m2_4kb_bdy_prt_ctrl(&mut self) -> M_4KB_BDY_PRT_CTRL_W<IOMMU_4KB_BDY_PRT_CTRL_SPEC> {
        M_4KB_BDY_PRT_CTRL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Master\\[i\\] 4 KB boundary protect control bit"]
    #[inline(always)]
    #[must_use]
    pub fn m3_4kb_bdy_prt_ctrl(&mut self) -> M_4KB_BDY_PRT_CTRL_W<IOMMU_4KB_BDY_PRT_CTRL_SPEC> {
        M_4KB_BDY_PRT_CTRL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Master\\[i\\] 4 KB boundary protect control bit"]
    #[inline(always)]
    #[must_use]
    pub fn m4_4kb_bdy_prt_ctrl(&mut self) -> M_4KB_BDY_PRT_CTRL_W<IOMMU_4KB_BDY_PRT_CTRL_SPEC> {
        M_4KB_BDY_PRT_CTRL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Master\\[i\\] 4 KB boundary protect control bit"]
    #[inline(always)]
    #[must_use]
    pub fn m5_4kb_bdy_prt_ctrl(&mut self) -> M_4KB_BDY_PRT_CTRL_W<IOMMU_4KB_BDY_PRT_CTRL_SPEC> {
        M_4KB_BDY_PRT_CTRL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Master\\[i\\] 4 KB boundary protect control bit"]
    #[inline(always)]
    #[must_use]
    pub fn m6_4kb_bdy_prt_ctrl(&mut self) -> M_4KB_BDY_PRT_CTRL_W<IOMMU_4KB_BDY_PRT_CTRL_SPEC> {
        M_4KB_BDY_PRT_CTRL_W::new(self, 6)
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
#[doc = "IOMMU 4KB Boundary Protect Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_4kb_bdy_prt_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_4kb_bdy_prt_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_4KB_BDY_PRT_CTRL_SPEC;
impl crate::RegisterSpec for IOMMU_4KB_BDY_PRT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_4kb_bdy_prt_ctrl::R`](R) reader structure"]
impl crate::Readable for IOMMU_4KB_BDY_PRT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_4kb_bdy_prt_ctrl::W`](W) writer structure"]
impl crate::Writable for IOMMU_4KB_BDY_PRT_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_4kb_bdy_prt_ctrl to value 0x7f"]
impl crate::Resettable for IOMMU_4KB_BDY_PRT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
