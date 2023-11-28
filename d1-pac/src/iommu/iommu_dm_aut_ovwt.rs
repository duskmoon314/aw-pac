#[doc = "Register `iommu_dm_aut_ovwt` reader"]
pub type R = crate::R<IOMMU_DM_AUT_OVWT_SPEC>;
#[doc = "Register `iommu_dm_aut_ovwt` writer"]
pub type W = crate::W<IOMMU_DM_AUT_OVWT_SPEC>;
#[doc = "Field `m_rd_aut_ovwt_ctrl[0-6]` reader - Master\\[i\\] read permission overwrite control"]
pub type M_RD_AUT_OVWT_CTRL_R = crate::BitReader<M_RD_AUT_OVWT_CTRL_A>;
#[doc = "Master\\[i\\] read permission overwrite control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_RD_AUT_OVWT_CTRL_A {
    #[doc = "0: The read-operation is permitted"]
    PERMITTED = 0,
    #[doc = "1: The read-operation is prohibited"]
    PROHIBITED = 1,
}
impl From<M_RD_AUT_OVWT_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: M_RD_AUT_OVWT_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl M_RD_AUT_OVWT_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_RD_AUT_OVWT_CTRL_A {
        match self.bits {
            false => M_RD_AUT_OVWT_CTRL_A::PERMITTED,
            true => M_RD_AUT_OVWT_CTRL_A::PROHIBITED,
        }
    }
    #[doc = "The read-operation is permitted"]
    #[inline(always)]
    pub fn is_permitted(&self) -> bool {
        *self == M_RD_AUT_OVWT_CTRL_A::PERMITTED
    }
    #[doc = "The read-operation is prohibited"]
    #[inline(always)]
    pub fn is_prohibited(&self) -> bool {
        *self == M_RD_AUT_OVWT_CTRL_A::PROHIBITED
    }
}
#[doc = "Field `m_rd_aut_ovwt_ctrl[0-6]` writer - Master\\[i\\] read permission overwrite control"]
pub type M_RD_AUT_OVWT_CTRL_W<'a, REG> = crate::BitWriter<'a, REG, M_RD_AUT_OVWT_CTRL_A>;
impl<'a, REG> M_RD_AUT_OVWT_CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The read-operation is permitted"]
    #[inline(always)]
    pub fn permitted(self) -> &'a mut crate::W<REG> {
        self.variant(M_RD_AUT_OVWT_CTRL_A::PERMITTED)
    }
    #[doc = "The read-operation is prohibited"]
    #[inline(always)]
    pub fn prohibited(self) -> &'a mut crate::W<REG> {
        self.variant(M_RD_AUT_OVWT_CTRL_A::PROHIBITED)
    }
}
#[doc = "Field `m_wt_aut_ovwt_ctrl[0-6]` reader - Master\\[i\\] write permission overwrite control"]
pub type M_WT_AUT_OVWT_CTRL_R = crate::BitReader<M_WT_AUT_OVWT_CTRL_A>;
#[doc = "Master\\[i\\] write permission overwrite control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_WT_AUT_OVWT_CTRL_A {
    #[doc = "0: The write-operation is permitted"]
    PERMITTED = 0,
    #[doc = "1: The write-operation is prohibited"]
    PROHIBITED = 1,
}
impl From<M_WT_AUT_OVWT_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: M_WT_AUT_OVWT_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl M_WT_AUT_OVWT_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_WT_AUT_OVWT_CTRL_A {
        match self.bits {
            false => M_WT_AUT_OVWT_CTRL_A::PERMITTED,
            true => M_WT_AUT_OVWT_CTRL_A::PROHIBITED,
        }
    }
    #[doc = "The write-operation is permitted"]
    #[inline(always)]
    pub fn is_permitted(&self) -> bool {
        *self == M_WT_AUT_OVWT_CTRL_A::PERMITTED
    }
    #[doc = "The write-operation is prohibited"]
    #[inline(always)]
    pub fn is_prohibited(&self) -> bool {
        *self == M_WT_AUT_OVWT_CTRL_A::PROHIBITED
    }
}
#[doc = "Field `m_wt_aut_ovwt_ctrl[0-6]` writer - Master\\[i\\] write permission overwrite control"]
pub type M_WT_AUT_OVWT_CTRL_W<'a, REG> = crate::BitWriter<'a, REG, M_WT_AUT_OVWT_CTRL_A>;
impl<'a, REG> M_WT_AUT_OVWT_CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The write-operation is permitted"]
    #[inline(always)]
    pub fn permitted(self) -> &'a mut crate::W<REG> {
        self.variant(M_WT_AUT_OVWT_CTRL_A::PERMITTED)
    }
    #[doc = "The write-operation is prohibited"]
    #[inline(always)]
    pub fn prohibited(self) -> &'a mut crate::W<REG> {
        self.variant(M_WT_AUT_OVWT_CTRL_A::PROHIBITED)
    }
}
#[doc = "Field `dm_aut_ovwt_enable` reader - Domain write/read permission overwrite enable"]
pub type DM_AUT_OVWT_ENABLE_R = crate::BitReader<DM_AUT_OVWT_ENABLE_A>;
#[doc = "Domain write/read permission overwrite enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DM_AUT_OVWT_ENABLE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<DM_AUT_OVWT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DM_AUT_OVWT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl DM_AUT_OVWT_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DM_AUT_OVWT_ENABLE_A {
        match self.bits {
            false => DM_AUT_OVWT_ENABLE_A::DISABLE,
            true => DM_AUT_OVWT_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DM_AUT_OVWT_ENABLE_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DM_AUT_OVWT_ENABLE_A::ENABLE
    }
}
#[doc = "Field `dm_aut_ovwt_enable` writer - Domain write/read permission overwrite enable"]
pub type DM_AUT_OVWT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, DM_AUT_OVWT_ENABLE_A>;
impl<'a, REG> DM_AUT_OVWT_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DM_AUT_OVWT_ENABLE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DM_AUT_OVWT_ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Master\\[i\\] read permission overwrite control\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `m0_rd_aut_ovwt_ctrl` field"]
    #[inline(always)]
    pub fn m_rd_aut_ovwt_ctrl(&self, n: u8) -> M_RD_AUT_OVWT_CTRL_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        M_RD_AUT_OVWT_CTRL_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    #[doc = "Bit 0 - Master\\[i\\] read permission overwrite control"]
    #[inline(always)]
    pub fn m0_rd_aut_ovwt_ctrl(&self) -> M_RD_AUT_OVWT_CTRL_R {
        M_RD_AUT_OVWT_CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Master\\[i\\] read permission overwrite control"]
    #[inline(always)]
    pub fn m1_rd_aut_ovwt_ctrl(&self) -> M_RD_AUT_OVWT_CTRL_R {
        M_RD_AUT_OVWT_CTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Master\\[i\\] read permission overwrite control"]
    #[inline(always)]
    pub fn m2_rd_aut_ovwt_ctrl(&self) -> M_RD_AUT_OVWT_CTRL_R {
        M_RD_AUT_OVWT_CTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Master\\[i\\] read permission overwrite control"]
    #[inline(always)]
    pub fn m3_rd_aut_ovwt_ctrl(&self) -> M_RD_AUT_OVWT_CTRL_R {
        M_RD_AUT_OVWT_CTRL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Master\\[i\\] read permission overwrite control"]
    #[inline(always)]
    pub fn m4_rd_aut_ovwt_ctrl(&self) -> M_RD_AUT_OVWT_CTRL_R {
        M_RD_AUT_OVWT_CTRL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Master\\[i\\] read permission overwrite control"]
    #[inline(always)]
    pub fn m5_rd_aut_ovwt_ctrl(&self) -> M_RD_AUT_OVWT_CTRL_R {
        M_RD_AUT_OVWT_CTRL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Master\\[i\\] read permission overwrite control"]
    #[inline(always)]
    pub fn m6_rd_aut_ovwt_ctrl(&self) -> M_RD_AUT_OVWT_CTRL_R {
        M_RD_AUT_OVWT_CTRL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Master\\[i\\] write permission overwrite control\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `m0_wt_aut_ovwt_ctrl` field"]
    #[inline(always)]
    pub fn m_wt_aut_ovwt_ctrl(&self, n: u8) -> M_WT_AUT_OVWT_CTRL_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        M_WT_AUT_OVWT_CTRL_R::new(((self.bits >> (n * 2 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - Master\\[i\\] write permission overwrite control"]
    #[inline(always)]
    pub fn m0_wt_aut_ovwt_ctrl(&self) -> M_WT_AUT_OVWT_CTRL_R {
        M_WT_AUT_OVWT_CTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Master\\[i\\] write permission overwrite control"]
    #[inline(always)]
    pub fn m1_wt_aut_ovwt_ctrl(&self) -> M_WT_AUT_OVWT_CTRL_R {
        M_WT_AUT_OVWT_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Master\\[i\\] write permission overwrite control"]
    #[inline(always)]
    pub fn m2_wt_aut_ovwt_ctrl(&self) -> M_WT_AUT_OVWT_CTRL_R {
        M_WT_AUT_OVWT_CTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Master\\[i\\] write permission overwrite control"]
    #[inline(always)]
    pub fn m3_wt_aut_ovwt_ctrl(&self) -> M_WT_AUT_OVWT_CTRL_R {
        M_WT_AUT_OVWT_CTRL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Master\\[i\\] write permission overwrite control"]
    #[inline(always)]
    pub fn m4_wt_aut_ovwt_ctrl(&self) -> M_WT_AUT_OVWT_CTRL_R {
        M_WT_AUT_OVWT_CTRL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Master\\[i\\] write permission overwrite control"]
    #[inline(always)]
    pub fn m5_wt_aut_ovwt_ctrl(&self) -> M_WT_AUT_OVWT_CTRL_R {
        M_WT_AUT_OVWT_CTRL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Master\\[i\\] write permission overwrite control"]
    #[inline(always)]
    pub fn m6_wt_aut_ovwt_ctrl(&self) -> M_WT_AUT_OVWT_CTRL_R {
        M_WT_AUT_OVWT_CTRL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 31 - Domain write/read permission overwrite enable"]
    #[inline(always)]
    pub fn dm_aut_ovwt_enable(&self) -> DM_AUT_OVWT_ENABLE_R {
        DM_AUT_OVWT_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Master\\[i\\] read permission overwrite control\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `m0_rd_aut_ovwt_ctrl` field"]
    #[inline(always)]
    #[must_use]
    pub fn m_rd_aut_ovwt_ctrl(&mut self, n: u8) -> M_RD_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        M_RD_AUT_OVWT_CTRL_W::new(self, n * 2)
    }
    #[doc = "Bit 0 - Master\\[i\\] read permission overwrite control"]
    #[inline(always)]
    #[must_use]
    pub fn m0_rd_aut_ovwt_ctrl(&mut self) -> M_RD_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        M_RD_AUT_OVWT_CTRL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Master\\[i\\] read permission overwrite control"]
    #[inline(always)]
    #[must_use]
    pub fn m1_rd_aut_ovwt_ctrl(&mut self) -> M_RD_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        M_RD_AUT_OVWT_CTRL_W::new(self, 2)
    }
    #[doc = "Bit 4 - Master\\[i\\] read permission overwrite control"]
    #[inline(always)]
    #[must_use]
    pub fn m2_rd_aut_ovwt_ctrl(&mut self) -> M_RD_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        M_RD_AUT_OVWT_CTRL_W::new(self, 4)
    }
    #[doc = "Bit 6 - Master\\[i\\] read permission overwrite control"]
    #[inline(always)]
    #[must_use]
    pub fn m3_rd_aut_ovwt_ctrl(&mut self) -> M_RD_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        M_RD_AUT_OVWT_CTRL_W::new(self, 6)
    }
    #[doc = "Bit 8 - Master\\[i\\] read permission overwrite control"]
    #[inline(always)]
    #[must_use]
    pub fn m4_rd_aut_ovwt_ctrl(&mut self) -> M_RD_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        M_RD_AUT_OVWT_CTRL_W::new(self, 8)
    }
    #[doc = "Bit 10 - Master\\[i\\] read permission overwrite control"]
    #[inline(always)]
    #[must_use]
    pub fn m5_rd_aut_ovwt_ctrl(&mut self) -> M_RD_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        M_RD_AUT_OVWT_CTRL_W::new(self, 10)
    }
    #[doc = "Bit 12 - Master\\[i\\] read permission overwrite control"]
    #[inline(always)]
    #[must_use]
    pub fn m6_rd_aut_ovwt_ctrl(&mut self) -> M_RD_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        M_RD_AUT_OVWT_CTRL_W::new(self, 12)
    }
    #[doc = "Master\\[i\\] write permission overwrite control\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `m0_wt_aut_ovwt_ctrl` field"]
    #[inline(always)]
    #[must_use]
    pub fn m_wt_aut_ovwt_ctrl(&mut self, n: u8) -> M_WT_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        M_WT_AUT_OVWT_CTRL_W::new(self, n * 2 + 1)
    }
    #[doc = "Bit 1 - Master\\[i\\] write permission overwrite control"]
    #[inline(always)]
    #[must_use]
    pub fn m0_wt_aut_ovwt_ctrl(&mut self) -> M_WT_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        M_WT_AUT_OVWT_CTRL_W::new(self, 1)
    }
    #[doc = "Bit 3 - Master\\[i\\] write permission overwrite control"]
    #[inline(always)]
    #[must_use]
    pub fn m1_wt_aut_ovwt_ctrl(&mut self) -> M_WT_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        M_WT_AUT_OVWT_CTRL_W::new(self, 3)
    }
    #[doc = "Bit 5 - Master\\[i\\] write permission overwrite control"]
    #[inline(always)]
    #[must_use]
    pub fn m2_wt_aut_ovwt_ctrl(&mut self) -> M_WT_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        M_WT_AUT_OVWT_CTRL_W::new(self, 5)
    }
    #[doc = "Bit 7 - Master\\[i\\] write permission overwrite control"]
    #[inline(always)]
    #[must_use]
    pub fn m3_wt_aut_ovwt_ctrl(&mut self) -> M_WT_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        M_WT_AUT_OVWT_CTRL_W::new(self, 7)
    }
    #[doc = "Bit 9 - Master\\[i\\] write permission overwrite control"]
    #[inline(always)]
    #[must_use]
    pub fn m4_wt_aut_ovwt_ctrl(&mut self) -> M_WT_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        M_WT_AUT_OVWT_CTRL_W::new(self, 9)
    }
    #[doc = "Bit 11 - Master\\[i\\] write permission overwrite control"]
    #[inline(always)]
    #[must_use]
    pub fn m5_wt_aut_ovwt_ctrl(&mut self) -> M_WT_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        M_WT_AUT_OVWT_CTRL_W::new(self, 11)
    }
    #[doc = "Bit 13 - Master\\[i\\] write permission overwrite control"]
    #[inline(always)]
    #[must_use]
    pub fn m6_wt_aut_ovwt_ctrl(&mut self) -> M_WT_AUT_OVWT_CTRL_W<IOMMU_DM_AUT_OVWT_SPEC> {
        M_WT_AUT_OVWT_CTRL_W::new(self, 13)
    }
    #[doc = "Bit 31 - Domain write/read permission overwrite enable"]
    #[inline(always)]
    #[must_use]
    pub fn dm_aut_ovwt_enable(&mut self) -> DM_AUT_OVWT_ENABLE_W<IOMMU_DM_AUT_OVWT_SPEC> {
        DM_AUT_OVWT_ENABLE_W::new(self, 31)
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
#[doc = "IOMMU Domain Authority Overwrite Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_dm_aut_ovwt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_dm_aut_ovwt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_DM_AUT_OVWT_SPEC;
impl crate::RegisterSpec for IOMMU_DM_AUT_OVWT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_dm_aut_ovwt::R`](R) reader structure"]
impl crate::Readable for IOMMU_DM_AUT_OVWT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_dm_aut_ovwt::W`](W) writer structure"]
impl crate::Writable for IOMMU_DM_AUT_OVWT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_dm_aut_ovwt to value 0"]
impl crate::Resettable for IOMMU_DM_AUT_OVWT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
