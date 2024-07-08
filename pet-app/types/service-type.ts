export interface ServiceContext  {
    followup_date: string | null;
    general_diagnosis: string;
    requires_followup: boolean;
    service_date: string;
    service_instance_id: string;
    service_reason: string;
    service_type: string[];
}