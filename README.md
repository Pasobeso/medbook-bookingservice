# Booking Service Routes

### EmptyResponseModel  
> คือจะไม่ส่ง data อะไรกลับมาเลย (ข้อมูลที่ส่งกลับมาเป็น None)

---

## ต้องการจะเพิ่ม slot เวลาของหมอ  
- **usecase** : add slot  
- **Endpoint** : `POST /slot-ops`

**Request**  
```rust
pub struct AddSlotDto {
    pub max_appointment_count: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}
```

**Response**  
```json
{
    "data": EmptyResponseModel,
    "message": "Some(String)"
}
```

---

## ต้องการจะแก้ slot เวลาของหมอ  
- **usecase** : edit slot  
- **Endpoint** : `PATCH /slot-ops/:slot_id`

**Request**  
```rust
pub struct EditSlotDto {
    pub max_appointment_count: Option<i32>,
    pub end_time: Option<NaiveDateTime>,
}
```

**Response**  
```json
{
    "data": EmptyResponseModel,
    "message": "Some(String)"
}
```

---

## ต้องการจะลบ slot เวลาทิ้ง  
- **usecase** : remove slot  
- **Endpoint** : `DELETE /slot-ops/:slot_id`

**Request**  
```
None
```

**Response**  
```json
{
    "data": EmptyResponseModel,
    "message": "Some(String)"
}
```

---

## คนไข้ต้องการจะจองหมอใน slot ใด ๆ  
- **usecase** : add appointment  
- **Endpoint** : `POST /appointment-ops`

**Request**  
```rust
pub struct AddAppointmentDto {
    pub slot_id: Uuid,
    pub patient_abnormal_symptom: String,
    pub patient_is_missed_medication: String,
    pub patient_blood_test_status: String,
    pub patient_is_overdue_medication: String,
    pub patient_is_partner_hiv_positive: String,
}
```

**Response**  
```json
{
    "data": EmptyResponseModel,
    "message": "Some(String)"
}
```

---

## คนไข้ต้องการจะเลื่อนนัด / แก้ไขรายละเอียดการนัดหมาย  
- **usecase** : edit appointment  
- **Endpoint** : `PATCH /appointment-ops`

**Request**  
```rust
pub struct EditAppointmentDto {
    pub slot_id: Option<Uuid>,
    pub patient_abnormal_symptom: Option<String>,
    pub patient_is_missed_medication: Option<String>,
    pub patient_blood_test_status: Option<String>,
    pub patient_is_overdue_medication: Option<String>,
    pub patient_is_partner_hiv_positive: Option<String>,
}
```

**Response**  
```json
{
    "data": EmptyResponseModel,
    "message": "Some(String)"
}
```

---

## คนไข้ต้องการจะยกเลิกนัด  
- **usecase** : remove appointment  
- **Endpoint** : `DELETE /appointment-ops/:appointment_id`

**Request**  
```
None
```

**Response**  
```json
{
    "data": EmptyResponseModel,
    "message": "Some(String)"
}
```

---

## Models

```rust
pub struct GetPatientScheduleResponseModel {
    pub schedules: Vec<ScheduleViewEntity>,
}

pub struct GetDoctorScheduleResponseModel {
    pub schedules: Vec<ScheduleViewEntity>,
}

pub struct ScheduleViewEntity {
    pub id: Uuid,
    pub slot_id: Uuid,
    pub patient_id: i32,
    pub patient_abnormal_symptom: String,
    pub patient_is_missed_medication: String,
    pub patient_blood_test_status: String,
    pub patient_is_overdue_medication: String,
    pub patient_is_partner_hiv_positive: String,
    pub status: String,
    pub doctor_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}
```

---

## คนไข้ต้องการจะดูตารางนัดหมายตัวเอง  
- **usecase** : get patient schedules  
- **Endpoint** : `GET /schedule-view/patient`

**Request**  
```
None
```

**Response**  
```json
{
    "data": GetPatientScheduleResponseModel,
    "message": "Some(String)"
}
```

---

## หมอต้องการจะดูตารางนัดหมายตัวเอง  
- **usecase** : get doctor schedules  
- **Endpoint** : `GET /schedule-view/doctor`

**Request**  
```
None
```

**Response**  
```json
{
    "data": GetDoctorScheduleResponseModel,
    "message": "Some(String)"
}
```

---

## SlotEntity และ Response Models  

```rust
pub struct SlotEntity {
    pub id: Uuid,
    pub doctor_id: i32,
    pub current_appointment_count: i32,
    pub max_appointment_count: i32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GetSlotsResponseModel {
    pub slots: Vec<SlotEntity>,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GetDoctorSlotsResponseModel {
    pub slots: Vec<SlotEntity>,
}
```

---

## คนไข้ต้องการจะดู slot ว่ามี slot ไหนว่างให้กดจองบ้าง หรือ หมอต้องการจะดูว่ามีหมอคนไหนจอง slot ไหนบ้าง  
- **usecase** : get slots  
- **Endpoint** : `GET /slot-view`

**Request**  
```
None
```

**Response**  
```json
{
    "data": GetSlotsResponseModel,
    "message": "Some(String)"
}
```

---

## หมอต้องการจะดู slot ของตัวเองว่าได้สร้าง slot ไหนไว้บ้าง  
- **usecase** : get slots  
- **Endpoint** : `GET /slot-view/view-my-slots`

**Request**  
```
None
```

**Response**  
```json
{
    "data": GetDoctorSlotsResponseModel,
    "message": "Some(String)"
}
```
