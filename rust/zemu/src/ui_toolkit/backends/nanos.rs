/*******************************************************************************
*   (c) 2021 Zondax GmbH
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License.
********************************************************************************/
use super::UIBackend;
use crate::ui_toolkit::{ZUI, strlen};

use arrayvec::ArrayString;

const KEY_SIZE: usize = 17 + 1;
//with null terminator
const MESSAGE_LINE_SIZE: usize = 17 + 1;
const MESSAGE_SIZE: usize = 2 * MESSAGE_LINE_SIZE + 1;

const INCLUDE_ACTIONS_AS_ITEMS: usize = 2;
const INCLUDE_ACTIONS_COUNT: usize = INCLUDE_ACTIONS_AS_ITEMS - 1;

pub struct NanoSBackend {
    key: ArrayString<KEY_SIZE>,

    message_line1: ArrayString<MESSAGE_LINE_SIZE>,
    message_line2: ArrayString<MESSAGE_LINE_SIZE>,
}

impl NanoSBackend {}

impl UIBackend<KEY_SIZE, MESSAGE_SIZE> for NanoSBackend {
    const INCLUDE_ACTIONS_COUNT: usize = INCLUDE_ACTIONS_COUNT;

    fn key_buf(&mut self) -> &mut ArrayString<KEY_SIZE> {
        &mut self.key
    }

    fn message_buf(&self) -> ArrayString<MESSAGE_SIZE> {
        ArrayString::new_const()
    }

    fn split_value_field(&mut self, message_buf: ArrayString<MESSAGE_SIZE>) {
        use core::fmt::Write;

        //clear inner message buffers
        self.message_line1.clear();
        self.message_line2.clear();

        //compute len and split `message_buf` at the max line size or at the total len
        // if the total len is less than the size of 1 line
        let len = strlen(message_buf.as_bytes());
        let split = core::cmp::min(MESSAGE_LINE_SIZE, len);
        let (line1, line2) = message_buf.split_at(split);

        //write the 2 lines, so if the message was small enough to fit
        // on the first line
        // then the second line will stay empty
        write!(&mut self.message_line1, "{}", line1);
        write!(&mut self.message_line2, "{}", line2);
    }

    fn view_error_show(&mut self) {
        todo!("UX_DISPLAY(view_error, view_prepro)");
    }

    fn view_review_show(ui: &mut ZUI<Self, KEY_SIZE, MESSAGE_SIZE>) {
        //reset ui struct
        ui.paging_init();

        match ui.review_update_data() {
            Ok(_) => {
                //UX_DISPLAY(view_review, view_prepro)
            },
            Err(_) => {
                ui.view_error_show()
            }
        }
    }
}
