export interface NotificationModel {
  id: string;
  updated_at: string;
  subject: Subject;
  repository: Repository;
}

interface Subject {
  title: string;
  url: string;
  type: string;
}

interface Repository {
  full_name: string;
  owner: Owner;
}

interface Owner {
  avatar_url: string;
}
