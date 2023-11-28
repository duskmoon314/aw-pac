#[doc = "Register `hc_fm_interval` reader"]
pub type R = crate::R<HC_FM_INTERVAL_SPEC>;
#[doc = "Register `hc_fm_interval` writer"]
pub type W = crate::W<HC_FM_INTERVAL_SPEC>;
#[doc = "Field `frame_interval` reader - FrameInterval\n\nThis specifies the interval between two consecutive SOFs in bit times. The nominal value is set to be 11,999. HCD should store the current value of this field before resetting HC. By setting the HostControllerReset field of as this will cause the HC to reset this field to its nominal value. HCD may choose to restore the stored value upon the completion of the Reset sequence."]
pub type FRAME_INTERVAL_R = crate::FieldReader<u16>;
#[doc = "Field `frame_interval` writer - FrameInterval\n\nThis specifies the interval between two consecutive SOFs in bit times. The nominal value is set to be 11,999. HCD should store the current value of this field before resetting HC. By setting the HostControllerReset field of as this will cause the HC to reset this field to its nominal value. HCD may choose to restore the stored value upon the completion of the Reset sequence."]
pub type FRAME_INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `fs_largest_data_packet` reader - FSLargestDataPacket\n\nThis field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame. The counter value represents the largest amount of data in bits which can be sent or received by the HC in a single transaction at any given time without causing scheduling overrun. The field value is calculated by the HCD."]
pub type FS_LARGEST_DATA_PACKET_R = crate::FieldReader<u16>;
#[doc = "Field `fs_largest_data_packet` writer - FSLargestDataPacket\n\nThis field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame. The counter value represents the largest amount of data in bits which can be sent or received by the HC in a single transaction at any given time without causing scheduling overrun. The field value is calculated by the HCD."]
pub type FS_LARGEST_DATA_PACKET_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `frame_interval_toggler` reader - FrameIntervalToggler HCD toggles this bit whenever it loads a new value to FrameInterval."]
pub type FRAME_INTERVAL_TOGGLER_R = crate::BitReader;
#[doc = "Field `frame_interval_toggler` writer - FrameIntervalToggler HCD toggles this bit whenever it loads a new value to FrameInterval."]
pub type FRAME_INTERVAL_TOGGLER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - FrameInterval\n\nThis specifies the interval between two consecutive SOFs in bit times. The nominal value is set to be 11,999. HCD should store the current value of this field before resetting HC. By setting the HostControllerReset field of as this will cause the HC to reset this field to its nominal value. HCD may choose to restore the stored value upon the completion of the Reset sequence."]
    #[inline(always)]
    pub fn frame_interval(&self) -> FRAME_INTERVAL_R {
        FRAME_INTERVAL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:30 - FSLargestDataPacket\n\nThis field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame. The counter value represents the largest amount of data in bits which can be sent or received by the HC in a single transaction at any given time without causing scheduling overrun. The field value is calculated by the HCD."]
    #[inline(always)]
    pub fn fs_largest_data_packet(&self) -> FS_LARGEST_DATA_PACKET_R {
        FS_LARGEST_DATA_PACKET_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - FrameIntervalToggler HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[inline(always)]
    pub fn frame_interval_toggler(&self) -> FRAME_INTERVAL_TOGGLER_R {
        FRAME_INTERVAL_TOGGLER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - FrameInterval\n\nThis specifies the interval between two consecutive SOFs in bit times. The nominal value is set to be 11,999. HCD should store the current value of this field before resetting HC. By setting the HostControllerReset field of as this will cause the HC to reset this field to its nominal value. HCD may choose to restore the stored value upon the completion of the Reset sequence."]
    #[inline(always)]
    #[must_use]
    pub fn frame_interval(&mut self) -> FRAME_INTERVAL_W<HC_FM_INTERVAL_SPEC> {
        FRAME_INTERVAL_W::new(self, 0)
    }
    #[doc = "Bits 16:30 - FSLargestDataPacket\n\nThis field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame. The counter value represents the largest amount of data in bits which can be sent or received by the HC in a single transaction at any given time without causing scheduling overrun. The field value is calculated by the HCD."]
    #[inline(always)]
    #[must_use]
    pub fn fs_largest_data_packet(&mut self) -> FS_LARGEST_DATA_PACKET_W<HC_FM_INTERVAL_SPEC> {
        FS_LARGEST_DATA_PACKET_W::new(self, 16)
    }
    #[doc = "Bit 31 - FrameIntervalToggler HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[inline(always)]
    #[must_use]
    pub fn frame_interval_toggler(&mut self) -> FRAME_INTERVAL_TOGGLER_W<HC_FM_INTERVAL_SPEC> {
        FRAME_INTERVAL_TOGGLER_W::new(self, 31)
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
#[doc = "OHCI Frame Interval Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_fm_interval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_fm_interval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC_FM_INTERVAL_SPEC;
impl crate::RegisterSpec for HC_FM_INTERVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc_fm_interval::R`](R) reader structure"]
impl crate::Readable for HC_FM_INTERVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc_fm_interval::W`](W) writer structure"]
impl crate::Writable for HC_FM_INTERVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_fm_interval to value 0x2edf"]
impl crate::Resettable for HC_FM_INTERVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2edf;
}
