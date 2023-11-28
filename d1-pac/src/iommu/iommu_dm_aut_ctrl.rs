#[doc = "Register `iommu_dm_aut_ctrl%s` reader"]
pub type R = crate::R<IOMMU_DM_AUT_CTRL_SPEC>;
#[doc = "Register `iommu_dm_aut_ctrl%s` writer"]
pub type W = crate::W<IOMMU_DM_AUT_CTRL_SPEC>;
#[doc = "Field `dm1_m_wt_aut_ctrl[0-6]` reader - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
pub type DM1_M_WT_AUT_CTRL_R = crate::BitReader<DM1_M_WT_AUT_CTRL_A>;
#[doc = "Domain \\[i + 1\\] write permission control for master \\[j\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DM1_M_WT_AUT_CTRL_A {
    #[doc = "0: The write-operation is permitted"]
    PERMITTED = 0,
    #[doc = "1: The write-operation is prohibited"]
    PROHIBITED = 1,
}
impl From<DM1_M_WT_AUT_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: DM1_M_WT_AUT_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl DM1_M_WT_AUT_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DM1_M_WT_AUT_CTRL_A {
        match self.bits {
            false => DM1_M_WT_AUT_CTRL_A::PERMITTED,
            true => DM1_M_WT_AUT_CTRL_A::PROHIBITED,
        }
    }
    #[doc = "The write-operation is permitted"]
    #[inline(always)]
    pub fn is_permitted(&self) -> bool {
        *self == DM1_M_WT_AUT_CTRL_A::PERMITTED
    }
    #[doc = "The write-operation is prohibited"]
    #[inline(always)]
    pub fn is_prohibited(&self) -> bool {
        *self == DM1_M_WT_AUT_CTRL_A::PROHIBITED
    }
}
#[doc = "Field `dm1_m_wt_aut_ctrl[0-6]` writer - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
pub type DM1_M_WT_AUT_CTRL_W<'a, REG> = crate::BitWriter<'a, REG, DM1_M_WT_AUT_CTRL_A>;
impl<'a, REG> DM1_M_WT_AUT_CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The write-operation is permitted"]
    #[inline(always)]
    pub fn permitted(self) -> &'a mut crate::W<REG> {
        self.variant(DM1_M_WT_AUT_CTRL_A::PERMITTED)
    }
    #[doc = "The write-operation is prohibited"]
    #[inline(always)]
    pub fn prohibited(self) -> &'a mut crate::W<REG> {
        self.variant(DM1_M_WT_AUT_CTRL_A::PROHIBITED)
    }
}
#[doc = "Field `dm1_m_rd_aut_ctrl[0-6]` reader - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
pub type DM1_M_RD_AUT_CTRL_R = crate::BitReader<DM1_M_RD_AUT_CTRL_A>;
#[doc = "Domain \\[i + 1\\] read permission control for master \\[j\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DM1_M_RD_AUT_CTRL_A {
    #[doc = "0: The read-operation is permitted"]
    PERMITTED = 0,
    #[doc = "1: The read-operation is prohibited"]
    PROHIBITED = 1,
}
impl From<DM1_M_RD_AUT_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: DM1_M_RD_AUT_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl DM1_M_RD_AUT_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DM1_M_RD_AUT_CTRL_A {
        match self.bits {
            false => DM1_M_RD_AUT_CTRL_A::PERMITTED,
            true => DM1_M_RD_AUT_CTRL_A::PROHIBITED,
        }
    }
    #[doc = "The read-operation is permitted"]
    #[inline(always)]
    pub fn is_permitted(&self) -> bool {
        *self == DM1_M_RD_AUT_CTRL_A::PERMITTED
    }
    #[doc = "The read-operation is prohibited"]
    #[inline(always)]
    pub fn is_prohibited(&self) -> bool {
        *self == DM1_M_RD_AUT_CTRL_A::PROHIBITED
    }
}
#[doc = "Field `dm1_m_rd_aut_ctrl[0-6]` writer - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
pub type DM1_M_RD_AUT_CTRL_W<'a, REG> = crate::BitWriter<'a, REG, DM1_M_RD_AUT_CTRL_A>;
impl<'a, REG> DM1_M_RD_AUT_CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The read-operation is permitted"]
    #[inline(always)]
    pub fn permitted(self) -> &'a mut crate::W<REG> {
        self.variant(DM1_M_RD_AUT_CTRL_A::PERMITTED)
    }
    #[doc = "The read-operation is prohibited"]
    #[inline(always)]
    pub fn prohibited(self) -> &'a mut crate::W<REG> {
        self.variant(DM1_M_RD_AUT_CTRL_A::PROHIBITED)
    }
}
#[doc = "Field `dm0_m_wt_aut_ctrl[0-6]` reader - Domain \\[i\\] write permission control for master \\[j\\]"]
pub type DM0_M_WT_AUT_CTRL_R = crate::BitReader<DM0_M_WT_AUT_CTRL_A>;
#[doc = "Domain \\[i\\] write permission control for master \\[j\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DM0_M_WT_AUT_CTRL_A {
    #[doc = "0: The write-operation is permitted"]
    PERMITTED = 0,
    #[doc = "1: The write-operation is prohibited"]
    PROHIBITED = 1,
}
impl From<DM0_M_WT_AUT_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: DM0_M_WT_AUT_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl DM0_M_WT_AUT_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DM0_M_WT_AUT_CTRL_A {
        match self.bits {
            false => DM0_M_WT_AUT_CTRL_A::PERMITTED,
            true => DM0_M_WT_AUT_CTRL_A::PROHIBITED,
        }
    }
    #[doc = "The write-operation is permitted"]
    #[inline(always)]
    pub fn is_permitted(&self) -> bool {
        *self == DM0_M_WT_AUT_CTRL_A::PERMITTED
    }
    #[doc = "The write-operation is prohibited"]
    #[inline(always)]
    pub fn is_prohibited(&self) -> bool {
        *self == DM0_M_WT_AUT_CTRL_A::PROHIBITED
    }
}
#[doc = "Field `dm0_m_wt_aut_ctrl[0-6]` writer - Domain \\[i\\] write permission control for master \\[j\\]"]
pub type DM0_M_WT_AUT_CTRL_W<'a, REG> = crate::BitWriter<'a, REG, DM0_M_WT_AUT_CTRL_A>;
impl<'a, REG> DM0_M_WT_AUT_CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The write-operation is permitted"]
    #[inline(always)]
    pub fn permitted(self) -> &'a mut crate::W<REG> {
        self.variant(DM0_M_WT_AUT_CTRL_A::PERMITTED)
    }
    #[doc = "The write-operation is prohibited"]
    #[inline(always)]
    pub fn prohibited(self) -> &'a mut crate::W<REG> {
        self.variant(DM0_M_WT_AUT_CTRL_A::PROHIBITED)
    }
}
#[doc = "Field `dm0_m_rd_aut_ctrl[0-6]` reader - Domain \\[i\\] read permission control for master \\[j\\]"]
pub type DM0_M_RD_AUT_CTRL_R = crate::BitReader<DM0_M_RD_AUT_CTRL_A>;
#[doc = "Domain \\[i\\] read permission control for master \\[j\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DM0_M_RD_AUT_CTRL_A {
    #[doc = "0: The read-operation is permitted"]
    PERMITTED = 0,
    #[doc = "1: The read-operation is prohibited"]
    PROHIBITED = 1,
}
impl From<DM0_M_RD_AUT_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: DM0_M_RD_AUT_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl DM0_M_RD_AUT_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DM0_M_RD_AUT_CTRL_A {
        match self.bits {
            false => DM0_M_RD_AUT_CTRL_A::PERMITTED,
            true => DM0_M_RD_AUT_CTRL_A::PROHIBITED,
        }
    }
    #[doc = "The read-operation is permitted"]
    #[inline(always)]
    pub fn is_permitted(&self) -> bool {
        *self == DM0_M_RD_AUT_CTRL_A::PERMITTED
    }
    #[doc = "The read-operation is prohibited"]
    #[inline(always)]
    pub fn is_prohibited(&self) -> bool {
        *self == DM0_M_RD_AUT_CTRL_A::PROHIBITED
    }
}
#[doc = "Field `dm0_m_rd_aut_ctrl[0-6]` writer - Domain \\[i\\] read permission control for master \\[j\\]"]
pub type DM0_M_RD_AUT_CTRL_W<'a, REG> = crate::BitWriter<'a, REG, DM0_M_RD_AUT_CTRL_A>;
impl<'a, REG> DM0_M_RD_AUT_CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The read-operation is permitted"]
    #[inline(always)]
    pub fn permitted(self) -> &'a mut crate::W<REG> {
        self.variant(DM0_M_RD_AUT_CTRL_A::PERMITTED)
    }
    #[doc = "The read-operation is prohibited"]
    #[inline(always)]
    pub fn prohibited(self) -> &'a mut crate::W<REG> {
        self.variant(DM0_M_RD_AUT_CTRL_A::PROHIBITED)
    }
}
impl R {
    #[doc = "Domain \\[i + 1\\] write permission control for master \\[j\\]\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `dm1_m0_wt_aut_ctrl` field"]
    #[inline(always)]
    pub fn dm1_m_wt_aut_ctrl(&self, n: u8) -> DM1_M_WT_AUT_CTRL_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        DM1_M_WT_AUT_CTRL_R::new(((self.bits >> (n * 2 + 17)) & 1) != 0)
    }
    #[doc = "Bit 17 - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm1_m0_wt_aut_ctrl(&self) -> DM1_M_WT_AUT_CTRL_R {
        DM1_M_WT_AUT_CTRL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm1_m1_wt_aut_ctrl(&self) -> DM1_M_WT_AUT_CTRL_R {
        DM1_M_WT_AUT_CTRL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm1_m2_wt_aut_ctrl(&self) -> DM1_M_WT_AUT_CTRL_R {
        DM1_M_WT_AUT_CTRL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm1_m3_wt_aut_ctrl(&self) -> DM1_M_WT_AUT_CTRL_R {
        DM1_M_WT_AUT_CTRL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm1_m4_wt_aut_ctrl(&self) -> DM1_M_WT_AUT_CTRL_R {
        DM1_M_WT_AUT_CTRL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm1_m5_wt_aut_ctrl(&self) -> DM1_M_WT_AUT_CTRL_R {
        DM1_M_WT_AUT_CTRL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm1_m6_wt_aut_ctrl(&self) -> DM1_M_WT_AUT_CTRL_R {
        DM1_M_WT_AUT_CTRL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Domain \\[i + 1\\] read permission control for master \\[j\\]\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `dm1_m0_rd_aut_ctrl` field"]
    #[inline(always)]
    pub fn dm1_m_rd_aut_ctrl(&self, n: u8) -> DM1_M_RD_AUT_CTRL_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        DM1_M_RD_AUT_CTRL_R::new(((self.bits >> (n * 2 + 17)) & 1) != 0)
    }
    #[doc = "Bit 17 - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm1_m0_rd_aut_ctrl(&self) -> DM1_M_RD_AUT_CTRL_R {
        DM1_M_RD_AUT_CTRL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm1_m1_rd_aut_ctrl(&self) -> DM1_M_RD_AUT_CTRL_R {
        DM1_M_RD_AUT_CTRL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm1_m2_rd_aut_ctrl(&self) -> DM1_M_RD_AUT_CTRL_R {
        DM1_M_RD_AUT_CTRL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm1_m3_rd_aut_ctrl(&self) -> DM1_M_RD_AUT_CTRL_R {
        DM1_M_RD_AUT_CTRL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm1_m4_rd_aut_ctrl(&self) -> DM1_M_RD_AUT_CTRL_R {
        DM1_M_RD_AUT_CTRL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm1_m5_rd_aut_ctrl(&self) -> DM1_M_RD_AUT_CTRL_R {
        DM1_M_RD_AUT_CTRL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm1_m6_rd_aut_ctrl(&self) -> DM1_M_RD_AUT_CTRL_R {
        DM1_M_RD_AUT_CTRL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Domain \\[i\\] write permission control for master \\[j\\]\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `dm0_m0_wt_aut_ctrl` field"]
    #[inline(always)]
    pub fn dm0_m_wt_aut_ctrl(&self, n: u8) -> DM0_M_WT_AUT_CTRL_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        DM0_M_WT_AUT_CTRL_R::new(((self.bits >> (n * 2 + 17)) & 1) != 0)
    }
    #[doc = "Bit 17 - Domain \\[i\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm0_m0_wt_aut_ctrl(&self) -> DM0_M_WT_AUT_CTRL_R {
        DM0_M_WT_AUT_CTRL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Domain \\[i\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm0_m1_wt_aut_ctrl(&self) -> DM0_M_WT_AUT_CTRL_R {
        DM0_M_WT_AUT_CTRL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Domain \\[i\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm0_m2_wt_aut_ctrl(&self) -> DM0_M_WT_AUT_CTRL_R {
        DM0_M_WT_AUT_CTRL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Domain \\[i\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm0_m3_wt_aut_ctrl(&self) -> DM0_M_WT_AUT_CTRL_R {
        DM0_M_WT_AUT_CTRL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Domain \\[i\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm0_m4_wt_aut_ctrl(&self) -> DM0_M_WT_AUT_CTRL_R {
        DM0_M_WT_AUT_CTRL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Domain \\[i\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm0_m5_wt_aut_ctrl(&self) -> DM0_M_WT_AUT_CTRL_R {
        DM0_M_WT_AUT_CTRL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Domain \\[i\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm0_m6_wt_aut_ctrl(&self) -> DM0_M_WT_AUT_CTRL_R {
        DM0_M_WT_AUT_CTRL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Domain \\[i\\] read permission control for master \\[j\\]\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `dm0_m0_rd_aut_ctrl` field"]
    #[inline(always)]
    pub fn dm0_m_rd_aut_ctrl(&self, n: u8) -> DM0_M_RD_AUT_CTRL_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        DM0_M_RD_AUT_CTRL_R::new(((self.bits >> (n * 2 + 17)) & 1) != 0)
    }
    #[doc = "Bit 17 - Domain \\[i\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm0_m0_rd_aut_ctrl(&self) -> DM0_M_RD_AUT_CTRL_R {
        DM0_M_RD_AUT_CTRL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Domain \\[i\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm0_m1_rd_aut_ctrl(&self) -> DM0_M_RD_AUT_CTRL_R {
        DM0_M_RD_AUT_CTRL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Domain \\[i\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm0_m2_rd_aut_ctrl(&self) -> DM0_M_RD_AUT_CTRL_R {
        DM0_M_RD_AUT_CTRL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Domain \\[i\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm0_m3_rd_aut_ctrl(&self) -> DM0_M_RD_AUT_CTRL_R {
        DM0_M_RD_AUT_CTRL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Domain \\[i\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm0_m4_rd_aut_ctrl(&self) -> DM0_M_RD_AUT_CTRL_R {
        DM0_M_RD_AUT_CTRL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Domain \\[i\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm0_m5_rd_aut_ctrl(&self) -> DM0_M_RD_AUT_CTRL_R {
        DM0_M_RD_AUT_CTRL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Domain \\[i\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    pub fn dm0_m6_rd_aut_ctrl(&self) -> DM0_M_RD_AUT_CTRL_R {
        DM0_M_RD_AUT_CTRL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Domain \\[i + 1\\] write permission control for master \\[j\\]\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `dm1_m0_wt_aut_ctrl` field"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m_wt_aut_ctrl(&mut self, n: u8) -> DM1_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        DM1_M_WT_AUT_CTRL_W::new(self, n * 2 + 17)
    }
    #[doc = "Bit 17 - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m0_wt_aut_ctrl(&mut self) -> DM1_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM1_M_WT_AUT_CTRL_W::new(self, 17)
    }
    #[doc = "Bit 19 - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m1_wt_aut_ctrl(&mut self) -> DM1_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM1_M_WT_AUT_CTRL_W::new(self, 19)
    }
    #[doc = "Bit 21 - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m2_wt_aut_ctrl(&mut self) -> DM1_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM1_M_WT_AUT_CTRL_W::new(self, 21)
    }
    #[doc = "Bit 23 - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m3_wt_aut_ctrl(&mut self) -> DM1_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM1_M_WT_AUT_CTRL_W::new(self, 23)
    }
    #[doc = "Bit 25 - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m4_wt_aut_ctrl(&mut self) -> DM1_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM1_M_WT_AUT_CTRL_W::new(self, 25)
    }
    #[doc = "Bit 27 - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m5_wt_aut_ctrl(&mut self) -> DM1_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM1_M_WT_AUT_CTRL_W::new(self, 27)
    }
    #[doc = "Bit 29 - Domain \\[i + 1\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m6_wt_aut_ctrl(&mut self) -> DM1_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM1_M_WT_AUT_CTRL_W::new(self, 29)
    }
    #[doc = "Domain \\[i + 1\\] read permission control for master \\[j\\]\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `dm1_m0_rd_aut_ctrl` field"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m_rd_aut_ctrl(&mut self, n: u8) -> DM1_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        DM1_M_RD_AUT_CTRL_W::new(self, n * 2 + 17)
    }
    #[doc = "Bit 17 - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m0_rd_aut_ctrl(&mut self) -> DM1_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM1_M_RD_AUT_CTRL_W::new(self, 17)
    }
    #[doc = "Bit 19 - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m1_rd_aut_ctrl(&mut self) -> DM1_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM1_M_RD_AUT_CTRL_W::new(self, 19)
    }
    #[doc = "Bit 21 - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m2_rd_aut_ctrl(&mut self) -> DM1_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM1_M_RD_AUT_CTRL_W::new(self, 21)
    }
    #[doc = "Bit 23 - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m3_rd_aut_ctrl(&mut self) -> DM1_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM1_M_RD_AUT_CTRL_W::new(self, 23)
    }
    #[doc = "Bit 25 - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m4_rd_aut_ctrl(&mut self) -> DM1_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM1_M_RD_AUT_CTRL_W::new(self, 25)
    }
    #[doc = "Bit 27 - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m5_rd_aut_ctrl(&mut self) -> DM1_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM1_M_RD_AUT_CTRL_W::new(self, 27)
    }
    #[doc = "Bit 29 - Domain \\[i + 1\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm1_m6_rd_aut_ctrl(&mut self) -> DM1_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM1_M_RD_AUT_CTRL_W::new(self, 29)
    }
    #[doc = "Domain \\[i\\] write permission control for master \\[j\\]\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `dm0_m0_wt_aut_ctrl` field"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m_wt_aut_ctrl(&mut self, n: u8) -> DM0_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        DM0_M_WT_AUT_CTRL_W::new(self, n * 2 + 17)
    }
    #[doc = "Bit 17 - Domain \\[i\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m0_wt_aut_ctrl(&mut self) -> DM0_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM0_M_WT_AUT_CTRL_W::new(self, 17)
    }
    #[doc = "Bit 19 - Domain \\[i\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m1_wt_aut_ctrl(&mut self) -> DM0_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM0_M_WT_AUT_CTRL_W::new(self, 19)
    }
    #[doc = "Bit 21 - Domain \\[i\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m2_wt_aut_ctrl(&mut self) -> DM0_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM0_M_WT_AUT_CTRL_W::new(self, 21)
    }
    #[doc = "Bit 23 - Domain \\[i\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m3_wt_aut_ctrl(&mut self) -> DM0_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM0_M_WT_AUT_CTRL_W::new(self, 23)
    }
    #[doc = "Bit 25 - Domain \\[i\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m4_wt_aut_ctrl(&mut self) -> DM0_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM0_M_WT_AUT_CTRL_W::new(self, 25)
    }
    #[doc = "Bit 27 - Domain \\[i\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m5_wt_aut_ctrl(&mut self) -> DM0_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM0_M_WT_AUT_CTRL_W::new(self, 27)
    }
    #[doc = "Bit 29 - Domain \\[i\\] write permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m6_wt_aut_ctrl(&mut self) -> DM0_M_WT_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM0_M_WT_AUT_CTRL_W::new(self, 29)
    }
    #[doc = "Domain \\[i\\] read permission control for master \\[j\\]\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `dm0_m0_rd_aut_ctrl` field"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m_rd_aut_ctrl(&mut self, n: u8) -> DM0_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        DM0_M_RD_AUT_CTRL_W::new(self, n * 2 + 17)
    }
    #[doc = "Bit 17 - Domain \\[i\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m0_rd_aut_ctrl(&mut self) -> DM0_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM0_M_RD_AUT_CTRL_W::new(self, 17)
    }
    #[doc = "Bit 19 - Domain \\[i\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m1_rd_aut_ctrl(&mut self) -> DM0_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM0_M_RD_AUT_CTRL_W::new(self, 19)
    }
    #[doc = "Bit 21 - Domain \\[i\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m2_rd_aut_ctrl(&mut self) -> DM0_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM0_M_RD_AUT_CTRL_W::new(self, 21)
    }
    #[doc = "Bit 23 - Domain \\[i\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m3_rd_aut_ctrl(&mut self) -> DM0_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM0_M_RD_AUT_CTRL_W::new(self, 23)
    }
    #[doc = "Bit 25 - Domain \\[i\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m4_rd_aut_ctrl(&mut self) -> DM0_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM0_M_RD_AUT_CTRL_W::new(self, 25)
    }
    #[doc = "Bit 27 - Domain \\[i\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m5_rd_aut_ctrl(&mut self) -> DM0_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM0_M_RD_AUT_CTRL_W::new(self, 27)
    }
    #[doc = "Bit 29 - Domain \\[i\\] read permission control for master \\[j\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dm0_m6_rd_aut_ctrl(&mut self) -> DM0_M_RD_AUT_CTRL_W<IOMMU_DM_AUT_CTRL_SPEC> {
        DM0_M_RD_AUT_CTRL_W::new(self, 29)
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
#[doc = "IOMMU Domain Authority Control \\[i\\] Register\n\nSoftware can set 15 different permission control types in IOMMU_DM_AUT_CTRL0-7. A default access control type is DOMAIN0. The read/write operation of DOMAIN1-15 is unlimited by default.\n\nSoftware needs to set the index of the permission control domain corresponding to the page table item in the bit\\[7:4\\] of the Level2 page table, the default value is 0 (use domain 0), that is, the read/write operation is not controlled.\n\nSetting REG_ARD_OVWT can mask the Domain control defined by IOMMU_DM_AUT_CTRL0-7. All Level2 page table type are covered by the type of REG_ARD_OVWT. The read/write operation is permitted by default.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iommu_dm_aut_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iommu_dm_aut_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMMU_DM_AUT_CTRL_SPEC;
impl crate::RegisterSpec for IOMMU_DM_AUT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iommu_dm_aut_ctrl::R`](R) reader structure"]
impl crate::Readable for IOMMU_DM_AUT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iommu_dm_aut_ctrl::W`](W) writer structure"]
impl crate::Writable for IOMMU_DM_AUT_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iommu_dm_aut_ctrl%s to value 0"]
impl crate::Resettable for IOMMU_DM_AUT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
