#[doc = "Register `prs_cap` reader"]
pub type R = crate::R<PRS_CAP_SPEC>;
#[doc = "Register `prs_cap` writer"]
pub type W = crate::W<PRS_CAP_SPEC>;
#[doc = "Field `ch_scap_on[0-3]` reader - Still capture control: Capture a single still image frame on channel \\[i\\].\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type CH_SCAP_ON_R = crate::BitReader<CH_SCAP_ON_A>;
#[doc = "Still capture control: Capture a single still image frame on channel \\[i\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH_SCAP_ON_A {
    #[doc = "0: Disable still capture"]
    DISABLE = 0,
    #[doc = "1: Enable still capture\n\nThe CSI module starts capturing image data at the start of the next frame. The CSI module captures only one frame of image data. This bit is self cleared and always reads as a 0."]
    ENABLE = 1,
}
impl From<CH_SCAP_ON_A> for bool {
    #[inline(always)]
    fn from(variant: CH_SCAP_ON_A) -> Self {
        variant as u8 != 0
    }
}
impl CH_SCAP_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH_SCAP_ON_A {
        match self.bits {
            false => CH_SCAP_ON_A::DISABLE,
            true => CH_SCAP_ON_A::ENABLE,
        }
    }
    #[doc = "Disable still capture"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH_SCAP_ON_A::DISABLE
    }
    #[doc = "Enable still capture\n\nThe CSI module starts capturing image data at the start of the next frame. The CSI module captures only one frame of image data. This bit is self cleared and always reads as a 0."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CH_SCAP_ON_A::ENABLE
    }
}
#[doc = "Field `ch_vcap_on[0-3]` reader - Video capture control: Capture the video image data stream on channel \\[i\\]."]
pub type CH_VCAP_ON_R = crate::BitReader<CH_VCAP_ON_A>;
#[doc = "Video capture control: Capture the video image data stream on channel \\[i\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH_VCAP_ON_A {
    #[doc = "0: Disable video capture\n\nIf video capture is in progress, the CSI stops capturing image data at the end of the current frame, and all of the current frame data is written to output FIFO."]
    DISABLE = 0,
    #[doc = "1: Enable video capture\n\nThe CSI starts capturing image data at the start of the next frame."]
    ENABLE = 1,
}
impl From<CH_VCAP_ON_A> for bool {
    #[inline(always)]
    fn from(variant: CH_VCAP_ON_A) -> Self {
        variant as u8 != 0
    }
}
impl CH_VCAP_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH_VCAP_ON_A {
        match self.bits {
            false => CH_VCAP_ON_A::DISABLE,
            true => CH_VCAP_ON_A::ENABLE,
        }
    }
    #[doc = "Disable video capture\n\nIf video capture is in progress, the CSI stops capturing image data at the end of the current frame, and all of the current frame data is written to output FIFO."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH_VCAP_ON_A::DISABLE
    }
    #[doc = "Enable video capture\n\nThe CSI starts capturing image data at the start of the next frame."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CH_VCAP_ON_A::ENABLE
    }
}
#[doc = "Field `ch_vcap_on[0-3]` writer - Video capture control: Capture the video image data stream on channel \\[i\\]."]
pub type CH_VCAP_ON_W<'a, REG> = crate::BitWriter<'a, REG, CH_VCAP_ON_A>;
impl<'a, REG> CH_VCAP_ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable video capture\n\nIf video capture is in progress, the CSI stops capturing image data at the end of the current frame, and all of the current frame data is written to output FIFO."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CH_VCAP_ON_A::DISABLE)
    }
    #[doc = "Enable video capture\n\nThe CSI starts capturing image data at the start of the next frame."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CH_VCAP_ON_A::ENABLE)
    }
}
#[doc = "Field `ch_fps_ds[0-3]` reader - Fps down sample"]
pub type CH_FPS_DS_R = crate::FieldReader<CH_FPS_DS_A>;
#[doc = "Fps down sample\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH_FPS_DS_A {
    #[doc = "0: no down sample"]
    NO_DOWN_SAMPLE = 0,
    #[doc = "2: 1/3 fps, only receives the first frame every 3 frames"]
    _1_3 = 2,
    #[doc = "3: 1/4 fps, only receives the first frame every 4 frames"]
    _1_4 = 3,
    #[doc = "4: 1/5 fps, only receives the first frame every 5 frames"]
    _1_5 = 4,
    #[doc = "5: 1/6 fps, only receives the first frame every 6 frames"]
    _1_6 = 5,
    #[doc = "6: 1/7 fps, only receives the first frame every 7 frames"]
    _1_7 = 6,
    #[doc = "7: 1/8 fps, only receives the first frame every 8 frames"]
    _1_8 = 7,
    #[doc = "8: 1/9 fps, only receives the first frame every 9 frames"]
    _1_9 = 8,
    #[doc = "9: 1/10 fps, only receives the first frame every 10 frames"]
    _1_10 = 9,
    #[doc = "10: 1/11 fps, only receives the first frame every 11 frames"]
    _1_11 = 10,
    #[doc = "11: 1/12 fps, only receives the first frame every 12 frames"]
    _1_12 = 11,
    #[doc = "12: 1/13 fps, only receives the first frame every 13 frames"]
    _1_13 = 12,
    #[doc = "13: 1/14 fps, only receives the first frame every 14 frames"]
    _1_14 = 13,
    #[doc = "14: 1/15 fps, only receives the first frame every 15 frames"]
    _1_15 = 14,
    #[doc = "15: 1/16 fps, only receives the first frame every 16 frames"]
    _1_16 = 15,
}
impl From<CH_FPS_DS_A> for u8 {
    #[inline(always)]
    fn from(variant: CH_FPS_DS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH_FPS_DS_A {
    type Ux = u8;
}
impl CH_FPS_DS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH_FPS_DS_A> {
        match self.bits {
            0 => Some(CH_FPS_DS_A::NO_DOWN_SAMPLE),
            2 => Some(CH_FPS_DS_A::_1_3),
            3 => Some(CH_FPS_DS_A::_1_4),
            4 => Some(CH_FPS_DS_A::_1_5),
            5 => Some(CH_FPS_DS_A::_1_6),
            6 => Some(CH_FPS_DS_A::_1_7),
            7 => Some(CH_FPS_DS_A::_1_8),
            8 => Some(CH_FPS_DS_A::_1_9),
            9 => Some(CH_FPS_DS_A::_1_10),
            10 => Some(CH_FPS_DS_A::_1_11),
            11 => Some(CH_FPS_DS_A::_1_12),
            12 => Some(CH_FPS_DS_A::_1_13),
            13 => Some(CH_FPS_DS_A::_1_14),
            14 => Some(CH_FPS_DS_A::_1_15),
            15 => Some(CH_FPS_DS_A::_1_16),
            _ => None,
        }
    }
    #[doc = "no down sample"]
    #[inline(always)]
    pub fn is_no_down_sample(&self) -> bool {
        *self == CH_FPS_DS_A::NO_DOWN_SAMPLE
    }
    #[doc = "1/3 fps, only receives the first frame every 3 frames"]
    #[inline(always)]
    pub fn is_1_3(&self) -> bool {
        *self == CH_FPS_DS_A::_1_3
    }
    #[doc = "1/4 fps, only receives the first frame every 4 frames"]
    #[inline(always)]
    pub fn is_1_4(&self) -> bool {
        *self == CH_FPS_DS_A::_1_4
    }
    #[doc = "1/5 fps, only receives the first frame every 5 frames"]
    #[inline(always)]
    pub fn is_1_5(&self) -> bool {
        *self == CH_FPS_DS_A::_1_5
    }
    #[doc = "1/6 fps, only receives the first frame every 6 frames"]
    #[inline(always)]
    pub fn is_1_6(&self) -> bool {
        *self == CH_FPS_DS_A::_1_6
    }
    #[doc = "1/7 fps, only receives the first frame every 7 frames"]
    #[inline(always)]
    pub fn is_1_7(&self) -> bool {
        *self == CH_FPS_DS_A::_1_7
    }
    #[doc = "1/8 fps, only receives the first frame every 8 frames"]
    #[inline(always)]
    pub fn is_1_8(&self) -> bool {
        *self == CH_FPS_DS_A::_1_8
    }
    #[doc = "1/9 fps, only receives the first frame every 9 frames"]
    #[inline(always)]
    pub fn is_1_9(&self) -> bool {
        *self == CH_FPS_DS_A::_1_9
    }
    #[doc = "1/10 fps, only receives the first frame every 10 frames"]
    #[inline(always)]
    pub fn is_1_10(&self) -> bool {
        *self == CH_FPS_DS_A::_1_10
    }
    #[doc = "1/11 fps, only receives the first frame every 11 frames"]
    #[inline(always)]
    pub fn is_1_11(&self) -> bool {
        *self == CH_FPS_DS_A::_1_11
    }
    #[doc = "1/12 fps, only receives the first frame every 12 frames"]
    #[inline(always)]
    pub fn is_1_12(&self) -> bool {
        *self == CH_FPS_DS_A::_1_12
    }
    #[doc = "1/13 fps, only receives the first frame every 13 frames"]
    #[inline(always)]
    pub fn is_1_13(&self) -> bool {
        *self == CH_FPS_DS_A::_1_13
    }
    #[doc = "1/14 fps, only receives the first frame every 14 frames"]
    #[inline(always)]
    pub fn is_1_14(&self) -> bool {
        *self == CH_FPS_DS_A::_1_14
    }
    #[doc = "1/15 fps, only receives the first frame every 15 frames"]
    #[inline(always)]
    pub fn is_1_15(&self) -> bool {
        *self == CH_FPS_DS_A::_1_15
    }
    #[doc = "1/16 fps, only receives the first frame every 16 frames"]
    #[inline(always)]
    pub fn is_1_16(&self) -> bool {
        *self == CH_FPS_DS_A::_1_16
    }
}
#[doc = "Field `ch_fps_ds[0-3]` writer - Fps down sample"]
pub type CH_FPS_DS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CH_FPS_DS_A>;
impl<'a, REG> CH_FPS_DS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no down sample"]
    #[inline(always)]
    pub fn no_down_sample(self) -> &'a mut crate::W<REG> {
        self.variant(CH_FPS_DS_A::NO_DOWN_SAMPLE)
    }
    #[doc = "1/3 fps, only receives the first frame every 3 frames"]
    #[inline(always)]
    pub fn _1_3(self) -> &'a mut crate::W<REG> {
        self.variant(CH_FPS_DS_A::_1_3)
    }
    #[doc = "1/4 fps, only receives the first frame every 4 frames"]
    #[inline(always)]
    pub fn _1_4(self) -> &'a mut crate::W<REG> {
        self.variant(CH_FPS_DS_A::_1_4)
    }
    #[doc = "1/5 fps, only receives the first frame every 5 frames"]
    #[inline(always)]
    pub fn _1_5(self) -> &'a mut crate::W<REG> {
        self.variant(CH_FPS_DS_A::_1_5)
    }
    #[doc = "1/6 fps, only receives the first frame every 6 frames"]
    #[inline(always)]
    pub fn _1_6(self) -> &'a mut crate::W<REG> {
        self.variant(CH_FPS_DS_A::_1_6)
    }
    #[doc = "1/7 fps, only receives the first frame every 7 frames"]
    #[inline(always)]
    pub fn _1_7(self) -> &'a mut crate::W<REG> {
        self.variant(CH_FPS_DS_A::_1_7)
    }
    #[doc = "1/8 fps, only receives the first frame every 8 frames"]
    #[inline(always)]
    pub fn _1_8(self) -> &'a mut crate::W<REG> {
        self.variant(CH_FPS_DS_A::_1_8)
    }
    #[doc = "1/9 fps, only receives the first frame every 9 frames"]
    #[inline(always)]
    pub fn _1_9(self) -> &'a mut crate::W<REG> {
        self.variant(CH_FPS_DS_A::_1_9)
    }
    #[doc = "1/10 fps, only receives the first frame every 10 frames"]
    #[inline(always)]
    pub fn _1_10(self) -> &'a mut crate::W<REG> {
        self.variant(CH_FPS_DS_A::_1_10)
    }
    #[doc = "1/11 fps, only receives the first frame every 11 frames"]
    #[inline(always)]
    pub fn _1_11(self) -> &'a mut crate::W<REG> {
        self.variant(CH_FPS_DS_A::_1_11)
    }
    #[doc = "1/12 fps, only receives the first frame every 12 frames"]
    #[inline(always)]
    pub fn _1_12(self) -> &'a mut crate::W<REG> {
        self.variant(CH_FPS_DS_A::_1_12)
    }
    #[doc = "1/13 fps, only receives the first frame every 13 frames"]
    #[inline(always)]
    pub fn _1_13(self) -> &'a mut crate::W<REG> {
        self.variant(CH_FPS_DS_A::_1_13)
    }
    #[doc = "1/14 fps, only receives the first frame every 14 frames"]
    #[inline(always)]
    pub fn _1_14(self) -> &'a mut crate::W<REG> {
        self.variant(CH_FPS_DS_A::_1_14)
    }
    #[doc = "1/15 fps, only receives the first frame every 15 frames"]
    #[inline(always)]
    pub fn _1_15(self) -> &'a mut crate::W<REG> {
        self.variant(CH_FPS_DS_A::_1_15)
    }
    #[doc = "1/16 fps, only receives the first frame every 16 frames"]
    #[inline(always)]
    pub fn _1_16(self) -> &'a mut crate::W<REG> {
        self.variant(CH_FPS_DS_A::_1_16)
    }
}
impl R {
    #[doc = "Still capture control: Capture a single still image frame on channel \\[i\\].\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_scap_on` field"]
    #[inline(always)]
    pub fn ch_scap_on(&self, n: u8) -> CH_SCAP_ON_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_SCAP_ON_R::new(((self.bits >> (n * 8)) & 1) != 0)
    }
    #[doc = "Bit 0 - Still capture control: Capture a single still image frame on channel \\[i\\]."]
    #[inline(always)]
    pub fn ch0_scap_on(&self) -> CH_SCAP_ON_R {
        CH_SCAP_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Still capture control: Capture a single still image frame on channel \\[i\\]."]
    #[inline(always)]
    pub fn ch1_scap_on(&self) -> CH_SCAP_ON_R {
        CH_SCAP_ON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Still capture control: Capture a single still image frame on channel \\[i\\]."]
    #[inline(always)]
    pub fn ch2_scap_on(&self) -> CH_SCAP_ON_R {
        CH_SCAP_ON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Still capture control: Capture a single still image frame on channel \\[i\\]."]
    #[inline(always)]
    pub fn ch3_scap_on(&self) -> CH_SCAP_ON_R {
        CH_SCAP_ON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Video capture control: Capture the video image data stream on channel \\[i\\].\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_vcap_on` field"]
    #[inline(always)]
    pub fn ch_vcap_on(&self, n: u8) -> CH_VCAP_ON_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_VCAP_ON_R::new(((self.bits >> (n * 8 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - Video capture control: Capture the video image data stream on channel \\[i\\]."]
    #[inline(always)]
    pub fn ch0_vcap_on(&self) -> CH_VCAP_ON_R {
        CH_VCAP_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - Video capture control: Capture the video image data stream on channel \\[i\\]."]
    #[inline(always)]
    pub fn ch1_vcap_on(&self) -> CH_VCAP_ON_R {
        CH_VCAP_ON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Video capture control: Capture the video image data stream on channel \\[i\\]."]
    #[inline(always)]
    pub fn ch2_vcap_on(&self) -> CH_VCAP_ON_R {
        CH_VCAP_ON_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 25 - Video capture control: Capture the video image data stream on channel \\[i\\]."]
    #[inline(always)]
    pub fn ch3_vcap_on(&self) -> CH_VCAP_ON_R {
        CH_VCAP_ON_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Fps down sample\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_fps_ds` field"]
    #[inline(always)]
    pub fn ch_fps_ds(&self, n: u8) -> CH_FPS_DS_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_FPS_DS_R::new(((self.bits >> (n * 8 + 2)) & 0x0f) as u8)
    }
    #[doc = "Bits 2:5 - Fps down sample"]
    #[inline(always)]
    pub fn ch0_fps_ds(&self) -> CH_FPS_DS_R {
        CH_FPS_DS_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 10:13 - Fps down sample"]
    #[inline(always)]
    pub fn ch1_fps_ds(&self) -> CH_FPS_DS_R {
        CH_FPS_DS_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - Fps down sample"]
    #[inline(always)]
    pub fn ch2_fps_ds(&self) -> CH_FPS_DS_R {
        CH_FPS_DS_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 26:29 - Fps down sample"]
    #[inline(always)]
    pub fn ch3_fps_ds(&self) -> CH_FPS_DS_R {
        CH_FPS_DS_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Video capture control: Capture the video image data stream on channel \\[i\\].\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_vcap_on` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_vcap_on(&mut self, n: u8) -> CH_VCAP_ON_W<PRS_CAP_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_VCAP_ON_W::new(self, n * 8 + 1)
    }
    #[doc = "Bit 1 - Video capture control: Capture the video image data stream on channel \\[i\\]."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_vcap_on(&mut self) -> CH_VCAP_ON_W<PRS_CAP_SPEC> {
        CH_VCAP_ON_W::new(self, 1)
    }
    #[doc = "Bit 9 - Video capture control: Capture the video image data stream on channel \\[i\\]."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_vcap_on(&mut self) -> CH_VCAP_ON_W<PRS_CAP_SPEC> {
        CH_VCAP_ON_W::new(self, 9)
    }
    #[doc = "Bit 17 - Video capture control: Capture the video image data stream on channel \\[i\\]."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_vcap_on(&mut self) -> CH_VCAP_ON_W<PRS_CAP_SPEC> {
        CH_VCAP_ON_W::new(self, 17)
    }
    #[doc = "Bit 25 - Video capture control: Capture the video image data stream on channel \\[i\\]."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_vcap_on(&mut self) -> CH_VCAP_ON_W<PRS_CAP_SPEC> {
        CH_VCAP_ON_W::new(self, 25)
    }
    #[doc = "Fps down sample\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_fps_ds` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_fps_ds(&mut self, n: u8) -> CH_FPS_DS_W<PRS_CAP_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_FPS_DS_W::new(self, n * 8 + 2)
    }
    #[doc = "Bits 2:5 - Fps down sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_fps_ds(&mut self) -> CH_FPS_DS_W<PRS_CAP_SPEC> {
        CH_FPS_DS_W::new(self, 2)
    }
    #[doc = "Bits 10:13 - Fps down sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_fps_ds(&mut self) -> CH_FPS_DS_W<PRS_CAP_SPEC> {
        CH_FPS_DS_W::new(self, 10)
    }
    #[doc = "Bits 18:21 - Fps down sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_fps_ds(&mut self) -> CH_FPS_DS_W<PRS_CAP_SPEC> {
        CH_FPS_DS_W::new(self, 18)
    }
    #[doc = "Bits 26:29 - Fps down sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_fps_ds(&mut self) -> CH_FPS_DS_W<PRS_CAP_SPEC> {
        CH_FPS_DS_W::new(self, 26)
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
#[doc = "Parser Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs_cap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs_cap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRS_CAP_SPEC;
impl crate::RegisterSpec for PRS_CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prs_cap::R`](R) reader structure"]
impl crate::Readable for PRS_CAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prs_cap::W`](W) writer structure"]
impl crate::Writable for PRS_CAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets prs_cap to value 0"]
impl crate::Resettable for PRS_CAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
