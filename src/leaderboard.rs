use serde::Deserialize;
use std::ffi::CString;
use std::os::raw::{c_int, c_char, c_uint, c_void};
use std::mem;

// calls out to JS
extern "C" {
	// index, rank, score, name
	fn update_leaderboard_entry(_: c_uint, _: c_uint, _: c_int, _: *mut c_char);
	fn clear_leaderboard();
}

#[no_mangle]
extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[derive(Deserialize, Debug,PartialEq, PartialOrd)]
pub struct LeaderboardEntry {
	id: String,
	rank: u32,
    name: String,
    score: i32,
	local: bool,
}

// this returns an unsorted list of leaderboard entries from the backend
pub fn get_leaderboard_entries(entries: &mut Vec<LeaderboardEntry>) {
	// temp until we get proper leaderboards
	let leaderboard = r#"[
		{
			"id": "0",
			"rank": 1,
			"name": "Jessica",
			"score": 300,
			"local": false
		},
		{
			"id": "1",
			"rank": 2,
			"name": "Zayden",
			"score": 130,
			"local": false
		},
		{
			"id": "2",
			"rank": 3,
			"name": "Esme",
			"score": 20,
			"local": false
		}
	]"#;
	let incoming : Vec<LeaderboardEntry> = serde_json::from_str(leaderboard).unwrap();
	for iiter in incoming {
		let i : LeaderboardEntry = iiter;
		if !entries.iter_mut().find(|x| x.id == i.id).is_none() {
			// update
		} else {
			entries.push(i);
		}
	}
}

// this will inject the local player and sort the leaderboard entries
pub fn prep_leaderboard_entries(entries: &mut Vec<LeaderboardEntry>, local_name: &str, local_score: i32) {

	// add or update local entry
	if let Some(index) = entries.iter().position(|e| e.local) {
		entries[index].score = local_score;
	} else {
		entries.push(LeaderboardEntry{
			id: "local".to_string(),
			rank: 0,
			name: local_name.to_string(),
			score: local_score,
			local: true,
		})
	}

	entries.sort_by(|a,b| b.score.cmp(&a.score));
	// TODO - update ranks to reflect below local
}

pub unsafe fn push_leaderboard_entries(entries: &Vec<LeaderboardEntry>) {
	clear_leaderboard();
	for (index, entry) in entries.iter().enumerate() {
		let s = CString::new(entry.name.clone()).unwrap();
		let name = s.into_raw();
		update_leaderboard_entry(index as u32, entry.rank, entry.score, name);
	}
}